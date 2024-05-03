use crate::diagnostics::BinData;
use csv::ReaderBuilder;
use serde::Deserialize;
use std::fs::{self, File};

/// Data for the Watt parameters: the energy of the neutron, and the _a_ and _b_ values.
#[derive(Debug, Deserialize)]
struct WattParameterData {
    energy: f64,
    a: f64,
    b: f64,
}

/// Data on the energy-dependent cross-section values.
#[derive(Debug, Deserialize)]
struct CrossSectionData {
    energy: f64,
    cross_section: f64,
}

/// Loading Watt parameters from a specified file path.
pub fn load_watt_parameters(file_path: &str) -> (Vec<f64>, Vec<f64>, Vec<f64>) {
    let csv_data = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let mut reader = ReaderBuilder::new().from_reader(csv_data.as_bytes());

    let mut energy_vector = Vec::<f64>::new();
    let mut a_vector = Vec::<f64>::new();
    let mut b_vector = Vec::<f64>::new();

    for result in reader.deserialize() {
        let record: WattParameterData = result.unwrap();
        energy_vector.push(record.energy);
        a_vector.push(record.a);
        b_vector.push(record.b);
    }

    (energy_vector, a_vector, b_vector)
}

/// Loading cross-sections from a specified file path.
pub fn load_cross_sections(file_path: &str) -> (Vec<f64>, Vec<f64>) {
    let csv_data = fs::read_to_string(file_path).expect(&format!(
        "Should have been able to read the file: {:?}",
        file_path
    ));
    let mut reader = ReaderBuilder::new().from_reader(csv_data.as_bytes());

    let mut energy_vector = Vec::<f64>::new();
    let mut cross_section_vector = Vec::<f64>::new();

    for result in reader.deserialize() {
        let record: CrossSectionData = result.unwrap();
        energy_vector.push(record.energy);
        cross_section_vector.push(record.cross_section);
    }

    (energy_vector, cross_section_vector)
}

/// Loads the bin data vector back into memory by serializing it.
pub fn load_bin_data_vector(file_path: &str) -> Vec<BinData> {
    let file = File::open(file_path).expect("Failed to open source data file.");

    let mut serde_reader = csv::Reader::from_reader(file);
    let mut bin_data_vector = Vec::<BinData>::new();

    for result in serde_reader.deserialize() {
        let record: BinData = result.expect("Failed to read source data file into BinData.");
        bin_data_vector.push(record);
    }
    bin_data_vector
}
