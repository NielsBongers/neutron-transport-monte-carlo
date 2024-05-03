use serde::{Deserialize, Serialize};

use crate::materials::material_data::MaterialData;
use crate::utils::data_handling::{get_watt_parameters, linear_interpolation};

use std::fmt;

mod b10;
mod be9;
mod fe54;
mod h1;
mod o16;
mod u235;
mod u238;
mod void;

/// All currently defined materials.
#[derive(PartialEq, Eq, Debug, Default, Copy, Clone, Serialize, Deserialize)]

pub enum MaterialNames {
    #[default]
    Void,
    H1,
    Be9,
    O16,
    Fe54,
    U235,
    U238,
    B10,
}

/// Mapping between the enum and indices.
pub fn map_enum_to_indices(material_name: &MaterialNames) -> usize {
    match material_name {
        MaterialNames::Void => 0,
        MaterialNames::H1 => 1,
        MaterialNames::Be9 => 2,
        MaterialNames::O16 => 3,
        MaterialNames::Fe54 => 4,
        MaterialNames::U235 => 5,
        MaterialNames::U238 => 6,
        MaterialNames::B10 => 7,
    }
}

/// Mapping between the indices and the enum.
pub fn map_indices_to_enum(material_index: usize) -> MaterialNames {
    match material_index {
        0 => MaterialNames::Void,
        1 => MaterialNames::H1,
        2 => MaterialNames::Be9,
        3 => MaterialNames::O16,
        4 => MaterialNames::Fe54,
        5 => MaterialNames::U235,
        6 => MaterialNames::U238,
        7 => MaterialNames::B10,
        _ => MaterialNames::Void,
    }
}

/// Used to instantiate materials from the ```MaterialData``` struct and return data on them for interactions.
#[derive(Default, Clone, Debug)]
pub struct MaterialProperties {
    pub number_density: f64,
    pub scattering: f64,
    pub absorption: f64,
    pub fission: f64,
    pub fissionable: bool,
    pub name: MaterialNames,
    pub watt_a: f64,
    pub watt_b: f64,
    pub nu_bar: f64,
    pub atomic_mass: f64,
}

impl fmt::Display for MaterialProperties {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let set_precision = 3;
        write!(f, "Material:\n\tMaterial name: {:?}\n\tScattering: {:.set_precision$} barn\n\tAbsorption: {:.set_precision$} barn\n\tFission: {:.set_precision$} barn\n\tFissionable: {}",
               self.name,
               self.scattering,
               self.absorption,
               self.fission,
               self.fissionable,
               set_precision = set_precision)
    }
}

impl MaterialProperties {
    /// Takes a ```MaterialData``` instance and uses linear interpolation to return information on the fission, scattering and absorption cross-sections for the neutron's energy.
    pub fn get_properties(&mut self, material_data: &MaterialData, energy: f64) -> () {
        self.number_density = material_data.number_density;

        (self.fission, _) = linear_interpolation(
            &material_data.energy_fission_cross_sections,
            &material_data.fission_cross_sections,
            energy,
        );

        (self.scattering, _) = linear_interpolation(
            &material_data.energy_scattering_cross_sections,
            &material_data.elastic_cross_sections,
            energy,
        );

        (self.absorption, _) = linear_interpolation(
            &material_data.energy_absorption_cross_sections,
            &material_data.absorption_cross_sections,
            energy,
        );

        (self.nu_bar, _) =
            linear_interpolation(&material_data.energy_nu_bar, &material_data.nu_bar, energy);

        (self.watt_a, self.watt_b) = get_watt_parameters(
            &material_data.energy_watt_parameters,
            &material_data.watt_parameters_a,
            &material_data.watt_parameters_b,
            energy,
        );

        self.fission = self.fission * 1e-28 * self.number_density;
        self.scattering = self.scattering * 1e-28 * self.number_density;
        self.absorption = self.absorption * 1e-28 * self.number_density;

        self.fissionable = material_data.fissionable;
        self.atomic_mass = material_data.atomic_mass;

        self.name = material_data.name;
    }

    /// Returns the total fission cross section, which for now is a combination of scattering, fission and absorption.
    pub fn total_cross_section(&self) -> f64 {
        let total_cross_section = self.scattering + self.fission + self.absorption;
        total_cross_section
    }
}

/// Creates and returns a vector with all different materials defined.
pub fn get_material_data_vector() -> Vec<MaterialData> {
    let void = MaterialData::get_void();
    let h1 = MaterialData::get_h1();
    let be9 = MaterialData::get_be9();
    let o16 = MaterialData::get_o16();
    let fe54 = MaterialData::get_fe54();
    let u235 = MaterialData::get_u235();
    let u238 = MaterialData::get_u238();
    let b10 = MaterialData::get_b10();

    // Setting void as index 0, so the default option.
    let material_data_vector: Vec<MaterialData> = vec![void, h1, be9, o16, fe54, u235, u238, b10];

    material_data_vector
}
