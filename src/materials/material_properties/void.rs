use crate::materials::material_properties::{MaterialData, MaterialNames};

impl MaterialData {
    pub fn get_void() -> MaterialData {
        // Fission
        let (energy_fission_cross_sections, fission_cross_sections) = (vec![0.0], vec![0.0]);

        // Scattering
        let (energy_scattering_cross_sections, elastic_cross_sections) = (vec![0.0], vec![0.0]);

        // Absorption
        let (energy_absorption_cross_sections, absorption_cross_sections) = (vec![0.0], vec![0.0]);

        // Nu bar
        let (energy_nu_bar, nu_bar) = (vec![0.0], vec![0.0]);

        // Watt parameters
        let (energy_watt_parameters, watt_parameters_a, watt_parameters_b) =
            (vec![0.0], vec![0.0], vec![0.0]);

        let number_density = 0.0;

        let name: MaterialNames = MaterialNames::Void;
        let atomic_mass = 0.0;

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
