use crate::materials::material_properties::MaterialNames;

/// Includes all required material data.
#[derive(Default, Debug)]
pub struct MaterialData {
    pub energy_fission_cross_sections: Vec<f64>,
    pub fission_cross_sections: Vec<f64>,

    pub energy_scattering_cross_sections: Vec<f64>,
    pub elastic_cross_sections: Vec<f64>,

    pub energy_absorption_cross_sections: Vec<f64>,
    pub absorption_cross_sections: Vec<f64>,

    pub energy_nu_bar: Vec<f64>,
    pub nu_bar: Vec<f64>,

    pub energy_watt_parameters: Vec<f64>,
    pub watt_parameters_a: Vec<f64>,
    pub watt_parameters_b: Vec<f64>,

    pub number_density: f64,
    pub atomic_mass: f64,

    pub thermal_conductivity: f64,
    pub density: f64,
    pub heat_capacity: f64,

    pub name: MaterialNames,
    pub fissionable: bool,
}
