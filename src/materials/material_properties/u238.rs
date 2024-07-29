use crate::materials::material_properties::{MaterialData, MaterialNames};
use crate::utils::data_loading::load_cross_sections;
use std::path::Path;

impl MaterialData {
    pub fn get_u238() -> MaterialData {
        // Fission
        let (energy_fission_cross_sections, fission_cross_sections) = (vec![0.0], vec![0.0]);

        // Scattering
        let (energy_scattering_cross_sections, elastic_cross_sections) =
            load_cross_sections(Path::new("data/u-238/u-238_aggregated_scattering.csv"));

        // Absorption
        let (energy_absorption_cross_sections, absorption_cross_sections) =
            load_cross_sections(Path::new("data/u-238/u-238_aggregated_absorption.csv"));

        // Nu bar
        let (energy_nu_bar, nu_bar) = (vec![0.0], vec![0.0]);

        // Watt parameters
        let (energy_watt_parameters, watt_parameters_a, watt_parameters_b) =
            (vec![0.0], vec![0.0], vec![0.0]);

        let number_density = 0.04833 * 1e24 * 1e6;

        let name: MaterialNames = MaterialNames::U238;
        let atomic_mass = 238.;

        let thermal_conductivity = 27.;
        let density = 19050.;
        let specific_heat = 116.;

        let fissionable: bool = false;

        MaterialData {
            energy_fission_cross_sections,
            fission_cross_sections,

            energy_scattering_cross_sections,
            elastic_cross_sections,

            energy_absorption_cross_sections,
            absorption_cross_sections,

            energy_nu_bar,
            nu_bar,

            energy_watt_parameters,
            watt_parameters_a,
            watt_parameters_b,

            number_density,
            atomic_mass,

            thermal_conductivity,
            density,
            heat_capacity: specific_heat,

            name,
            fissionable,
        }
    }
}
