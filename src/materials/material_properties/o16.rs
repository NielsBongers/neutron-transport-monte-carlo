use crate::materials::material_properties::{MaterialData, MaterialNames};
use crate::utils::data_loading::load_cross_sections;
use std::path::Path;

impl MaterialData {
    pub fn get_o16() -> MaterialData {
        // Fission
        let (energy_fission_cross_sections, fission_cross_sections) = (vec![0.0], vec![0.0]);

        // Scattering
        let (energy_scattering_cross_sections, elastic_cross_sections) =
            load_cross_sections(Path::new("data/o-16/o-16_aggregated_scattering.csv"));

        // Absorption
        let (energy_absorption_cross_sections, absorption_cross_sections) =
            load_cross_sections(Path::new("data/o-16/o-16_aggregated_absorption.csv"));

        // Nu bar
        let (energy_nu_bar, nu_bar) = (vec![0.0], vec![0.0]);

        // Watt parameters
        let (energy_watt_parameters, watt_parameters_a, watt_parameters_b) =
            (vec![0.0], vec![0.0], vec![0.0]);

        // This is a bit of a hack.
        // Hydrogen is essentially irrelevant as a gas, because the number density is ~ 0.
        // The only case I want to use it is for water.
        // Therefore, I take the number density of water and take 2/3 of that for the 2 H, and 1/3 for O in the fractions.
        let number_density = 3.34272 * 1e28;

        let name: MaterialNames = MaterialNames::O16;
        let atomic_mass = 16.;

        let thermal_conductivity = 0.0;
        let density = 0.0;
        let specific_heat = 0.0;

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
