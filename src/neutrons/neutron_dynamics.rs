use crate::materials::material_properties::MaterialProperties;
use crate::neutrons::watt_distribution::rejection_sample_watt;
use crate::neutrons::Neutron;
use crate::utils::vectors::Vec3D;
use rand::Rng;
use std::fmt;

use log::warn;

/// Defined interaction types. We have Fission, Scattering, Absorption, Escaped (if the neutron exits a defined bound) and None, if no interaction occurs and the neutron passes unimpeded.
#[derive(PartialEq, Eq)]
pub enum InteractionTypes {
    Fission,
    Scattering,
    Absorption,
    Escaped,
    None,
}

impl fmt::Display for Neutron {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let c = 299792458.;
        let set_precision = 3;
        write!(f, "Neutron:\n\tEnergy: {:.set_precision$} MeV,\n\tVelocity: {:.set_precision$} m/s ({:.set_precision$} km/s, {:.set_precision$} c),\n\tTime step: {:.set_precision$} ns,\n\tCreation time: {:.set_precision$} ns,\n\tCurrent time: {:.set_precision$} ns,\n\tPosition: ({:.set_precision$}, {:.set_precision$}, {:.set_precision$}) m,\n\tDirection: ({:.set_precision$}, {:.set_precision$}, {:.set_precision$}) m,\n\tGeneration number: {}\n",
            self.energy / 1e6, self.velocity, self.velocity / 1e3, self.velocity / c, self.time_step*1e9, self.creation_time * 1e9, self.current_time * 1e9,
            self.position.x, self.position.y, self.position.z,
            self.direction.x, self.direction.y, self.direction.z,
            self.generation_number)
    }
}

impl Neutron {
    /// Initialization of the neutron. Includes constants (like _m_<sub>neutron</sub> and _q_), evolution values (such as generation times and numbers) and calculates values for the neutron (like _v_ and _Δt_).
    pub fn initialize(
        &mut self,
        parent_neutron: &Neutron,
        watt_a: f64,
        watt_b: f64,
        rng: &mut rand::rngs::SmallRng,
    ) -> () {
        let neutron_mass = 1.67492749804e-27;
        self.distance_step = 0.001;
        let q = 1.60218e-19;

        self.energy = self.get_energy(watt_a, watt_b, rng);

        self.creation_time = parent_neutron.current_time;
        self.current_time = parent_neutron.current_time;
        self.position = parent_neutron.position;
        self.generation_number = parent_neutron.generation_number + 1;

        self.velocity = f64::sqrt(2.0 * self.energy * q / neutron_mass);
        self.time_step = self.distance_step / self.velocity;

        self.direction = Vec3D::random_unit_vector(rng);
    }

    /// Interpolates the Watt parameters for the current energy, then uses rejection sampling to create a new neutron.
    pub fn get_energy(&mut self, watt_a: f64, watt_b: f64, rng: &mut rand::rngs::SmallRng) -> f64 {
        match rejection_sample_watt(watt_a, watt_b, rng) {
            Some(energy) => energy,
            None => {
                warn!("Rejection sampling failed - returning 1e6 eV. Results may be incorrect.");
                1e6
            }
        }
    }

    /// Isotropic scattering of the neutron.
    pub fn scatter(
        &mut self,
        atomic_mass: f64,
        rng: &mut rand::rngs::SmallRng,
        maximum_neutron_energy_difference: f64,
    ) -> () {
        let new_direction: Vec3D = Vec3D::random_unit_vector(rng);
        let cos_theta = self.direction.dot(new_direction);

        // let angle = cos_theta.acos() / (2. * 3.1415) * 360.;
        // debug!(
        //     "Angle: {}. Atomic mass: {}, Energy lost: {}",
        //     angle,
        //     atomic_mass,
        //     1.0 - (atomic_mass.powi(2) + 1. + 2.0 * atomic_mass * cos_theta)
        //         / (atomic_mass + 1.).powi(2)
        // );

        self.direction = new_direction;

        let remaining_energy_fraction =
            (atomic_mass.powi(2) + 1. + 2.0 * atomic_mass * cos_theta) / (atomic_mass + 1.).powi(2);

        if (1.0 - remaining_energy_fraction) > maximum_neutron_energy_difference {
            self.has_scattered = true;
            // debug!(
            //     "Relevant scattering! Updating! {} compared to {}",
            //     (1.0 - remaining_energy_fraction),
            //     maximum_neutron_energy_difference
            // )
        }

        self.energy *= remaining_energy_fraction;

        // debug!(
        //     "Angle: {}, Energy decrease: {}",
        //     angle,
        //     1. - (atomic_mass.powi(2) + 1. + 2.0 * atomic_mass * cos_theta)
        //         / (atomic_mass + 1.).powi(2)
        // );
    }

    /// Translation of the neutron in the current movement direction.
    pub fn translate(&mut self) -> () {
        self.position.x += self.direction.x * self.distance_step;
        self.position.y += self.direction.y * self.distance_step;
        self.position.z += self.direction.z * self.distance_step;
        // debug!("{}", self.direction.norm());
    }

    /// Calculates the number of fission neutrons in U-235.
    pub fn get_neutron_fission_count(&self, nu_bar: f64, rng: &mut rand::rngs::SmallRng) -> i32 {
        let floored_nu_bar = nu_bar.floor();
        let floating_point_difference = nu_bar - floored_nu_bar;

        if floating_point_difference > 0.0 {
            if rng.gen::<f64>() <= floating_point_difference {
                return floored_nu_bar as i32 + 1;
            } else {
                return floored_nu_bar as i32;
            }
        } else {
            return floored_nu_bar as i32;
        }
    }

    /// Interactions between a material instance from ```MaterialProperties``` and the neutron. All interactions (fission, absorption, scattering) are energy-dependent and continuous.
    pub fn interact(
        &self,
        material: &MaterialProperties,
        composition_total_cross_section: f64,
        simulation_range_squared: f64,
        rng: &mut rand::rngs::SmallRng,
    ) -> InteractionTypes {
        if self.position.norm_squared() >= simulation_range_squared {
            return InteractionTypes::Escaped;
        }

        let interaction_criterion = rng.gen::<f64>();
        let interaction_probability =
            1.0 - f64::exp(-self.distance_step * composition_total_cross_section);

        // if composition_total_cross_section > 0.0 {
        //     debug!("Interaction probability: {}", interaction_probability);
        // }

        if interaction_criterion <= interaction_probability {
            let interaction_type_criterion = rng.gen::<f64>();

            // Order:
            // 0 < Fission <= Scattering <= Absorption < 1
            // Random number here is [0, 1), so exclusive 1. The else-clause is just there because otherwise Rust complains.
            let fission_probability = material.fission / material.total_cross_section();
            let scatter_probability = material.scattering / material.total_cross_section();
            let absorption_probability = material.absorption / material.total_cross_section();

            if interaction_type_criterion >= 0.0 && interaction_type_criterion < fission_probability
            {
                return InteractionTypes::Fission;
            } else if interaction_type_criterion >= fission_probability
                && interaction_type_criterion < fission_probability + scatter_probability
            {
                return InteractionTypes::Scattering;
            } else if interaction_type_criterion >= fission_probability + scatter_probability
                && interaction_type_criterion
                    < fission_probability + scatter_probability + absorption_probability
            {
                return InteractionTypes::Absorption;
            } else {
                return InteractionTypes::Absorption;
            }
        } else {
            return InteractionTypes::None;
        }
    }

    /// Debugging function to set the neutron's position.
    pub fn _set_position(&mut self, x: f64, y: f64, z: f64) -> () {
        self.position.x = x;
        self.position.y = y;
        self.position.z = z;
    }
}
