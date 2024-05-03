use crate::diagnostics::geometry_diagnostics::GeometryDiagnostics;
use crate::diagnostics::halt_causes::SimulationHaltCauses;
use crate::utils::vectors::Vec3D;

use crate::diagnostics::{BinData, NeutronDiagnostics};

impl BinData {
    // Function to add two BinData instances
    pub fn add(&mut self, other: &BinData) {
        self.neutron_count += other.neutron_count;
        self.fission_count += other.fission_count;
    }
}

impl NeutronDiagnostics {
    pub fn new(
        track_creation: bool,
        track_positions: bool,
        track_energies: bool,
        track_bins: bool,
        track_from_generation: i64,
        bin_parameters: GeometryDiagnostics,
        initial_neutron_count: i64,
    ) -> NeutronDiagnostics {
        let bin_data = BinData::default();

        let neutron_position_bins = vec![
            bin_data;
            (bin_parameters.length_count + 1)
                * (bin_parameters.depth_count + 1)
                * (bin_parameters.height_count + 1)
        ];

        let max_generation_value = 0;
        let averaged_k = 0.0;
        let halt_cause = SimulationHaltCauses::default();
        let total_neutrons_tracked = 0;
        let total_fissions = 0;
        let average_power: f64 = 0.0;

        NeutronDiagnostics {
            creation_times: Vec::<f64>::new(),
            generation_number: Vec::<i64>::new(),
            neutron_positions: Vec::<Vec3D>::new(),
            neutron_energies: Vec::<(f64, f64)>::new(),
            generation_counts: Vec::<i64>::new(),
            bin_parameters,
            neutron_position_bins,
            track_creation,
            track_positions,
            track_energies,
            track_bins,
            max_generation_value,
            averaged_k,
            halt_cause,
            initial_neutron_count,
            total_neutrons_tracked,
            total_fissions,
            track_from_generation,
            power_generated: average_power,
        }
    }

    pub fn track_creation(&mut self, creation_time: f64, generation_number: i64) {
        if self.track_creation && generation_number >= self.track_from_generation {
            self.creation_times.push(creation_time);
            self.generation_number.push(generation_number);
        }
    }

    pub fn track_neutron_position(&mut self, generation_number: i64, neutron_position: Vec3D) {
        if self.track_positions && generation_number >= self.track_from_generation {
            self.neutron_positions.push(neutron_position);
        }
    }

    pub fn track_neutron_energies(
        &mut self,
        generation_number: i64,
        timestamp: f64,
        neutron_energy: f64,
    ) {
        // debug!("Neutron energy: {}", neutron_energy);
        if self.track_energies && generation_number >= self.track_from_generation {
            self.neutron_energies.push((timestamp, neutron_energy));
        }
    }

    pub fn get_current_bin(&self, neutron_position: Vec3D) -> Option<usize> {
        if !(neutron_position.x < self.bin_parameters.x_min
            || neutron_position.x > self.bin_parameters.x_max
            || neutron_position.y < self.bin_parameters.y_min
            || neutron_position.y > self.bin_parameters.y_max
            || neutron_position.z < self.bin_parameters.z_min
            || neutron_position.z > self.bin_parameters.z_max)
        {
            let x_bin = ((neutron_position.x - self.bin_parameters.x_min)
                / (self.bin_parameters.x_max - self.bin_parameters.x_min)
                * self.bin_parameters.length_count as f64) as usize;
            let y_bin = ((neutron_position.y - self.bin_parameters.y_min)
                / (self.bin_parameters.y_max - self.bin_parameters.y_min)
                * self.bin_parameters.depth_count as f64) as usize;
            let z_bin = ((neutron_position.z - self.bin_parameters.z_min)
                / (self.bin_parameters.z_max - self.bin_parameters.z_min)
                * self.bin_parameters.height_count as f64) as usize;

            let current_bin = x_bin
                + y_bin * self.bin_parameters.length_count
                + z_bin * self.bin_parameters.length_count * self.bin_parameters.depth_count;

            return Some(current_bin);
        }
        None
    }

    pub fn track_neutron_bin_presence(&mut self, generation_number: i64, neutron_position: Vec3D) {
        if self.track_bins && generation_number >= self.track_from_generation {
            if let Some(current_bin) = self.get_current_bin(neutron_position) {
                self.neutron_position_bins[current_bin].neutron_count += 1
            }
        }
    }

    pub fn track_neutron_bin_fission(&mut self, generation_number: i64, neutron_position: Vec3D) {
        if self.track_bins && generation_number >= self.track_from_generation {
            if let Some(current_bin) = self.get_current_bin(neutron_position) {
                self.neutron_position_bins[current_bin].fission_count += 1
            }
        }
    }

    pub fn track_simulation_halt(
        &mut self,
        total_neutron_count: i64,
        neutron_generation: i64,
        neutron_generation_cap: i64,
        neutron_count_cap: i64,
    ) {
        if total_neutron_count > neutron_count_cap {
            self.halt_cause = SimulationHaltCauses::HitNeutronCap;
        }
        if neutron_generation > neutron_generation_cap {
            self.halt_cause = SimulationHaltCauses::HitGenerationCap;
        }
        if total_neutron_count == 0 {
            self.halt_cause = SimulationHaltCauses::NoNeutrons;
        }
        self.max_generation_value = neutron_generation;
    }
}
