use crate::materials::material_properties::{MaterialData, MaterialNames};
use crate::utils::data_loading::{load_cross_sections, load_watt_parameters};

impl MaterialData {
    pub fn get_u235() -> MaterialData {
        // Fission
        let (energy_fission_cross_sections, fission_cross_sections) =
            load_cross_sections("data/u-235/u-235_fission.csv");

        // Scattering
        let (energy_scattering_cross_sections, elastic_cross_sections) =
            load_cross_sections("data/u-235/u-235_aggregated_scattering.csv");

        // Absorption
        let (energy_absorption_cross_sections, absorption_cross_sections) =
            load_cross_sections("data/u-235/u-235_aggregated_absorption.csv");

        // Nu bar
        let (energy_nu_bar, nu_bar) = load_cross_sections("data/u-235/u-235_nu_bar.csv");

        // Watt parameters
        let (energy_watt_parameters, watt_parameters_a, watt_parameters_b) =
            load_watt_parameters("data/u-235/u-235_watt_parameters.csv");

        let number_density = 0.04833 * 1e24 * 1e6;

        let name: MaterialNames = MaterialNames::U235;
        let atomic_mass = 235.;
        let fissionable: bool = true;

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
            name,
            fissionable,
        }
    }
}
