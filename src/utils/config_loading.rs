use crate::utils::vectors::Vec3D;
use serde::Deserialize;
use std::fs;
use toml;

/// Struct containing the three parameters we can load in. Used by serde and returned to the main function.
#[derive(Deserialize)]
pub struct Config {
    pub parallelization_parameters: ParallelizationParametersTOML,
    pub simulation_parameters: SimulationParametersTOML,
    pub heat_diffusion_parameters: HeatDiffusionParametersTOML,
    pub bin_parameters: BinParametersTOML,
    pub plot_parameters: PlotParametersTOML,
}

/// Parameters for multithreading
#[derive(Deserialize)]
pub struct ParallelizationParametersTOML {
    pub number_threads: i64,
    pub simulations_per_thread: i64,
}

/// Parameters specific to the simulation.
#[derive(Deserialize)]
pub struct SimulationParametersTOML {
    pub neutron_initial_position: Vec3D,
    pub neutron_generation_cap: i64,
    pub neutron_count_cap: i64,
    pub initial_neutron_count: i64,
    pub enforce_maximum_neutron_count: bool,
    pub track_creation: bool,
    pub track_bins: bool,
    pub track_from_generation: i64,
    pub plot_geometry: bool,
    pub write_results: bool,
    pub halt_time: Option<f64>,
    pub maximum_neutron_energy_difference: f64,
    pub geometries_path: String,
    pub model_heat_diffusion: bool,
}

/// Parameters for heat diffusion modelling.
#[derive(Deserialize)]
pub struct HeatDiffusionParametersTOML {
    pub source_data_file: String,
    pub neutron_multiplier: f64,
    pub thermal_conductivity: f64,
    pub density: f64,
    pub heat_capacity: f64,
    pub external_temperature: f64,
    pub time_step: f64,
    pub t_end: f64,
    pub write_interval: i64,
    pub convective_heat_transfer_coefficient: f64,
}

/// Parameters for the bins for neutron behavior.
#[derive(Deserialize)]
pub struct BinParametersTOML {
    pub length_count: usize,
    pub depth_count: usize,
    pub height_count: usize,
    pub total_length: f64,
    pub total_depth: f64,
    pub total_height: f64,
    pub center: Vec3D,
}

/// Parameters for plotting.
#[derive(Deserialize)]
pub struct PlotParametersTOML {
    pub length_count: usize,
    pub depth_count: usize,
    pub height_count: usize,
    pub total_length: f64,
    pub total_depth: f64,
    pub total_height: f64,
    pub center: Vec3D,
}

/// Loading the config file into a ```Config``` object and returning that.
pub fn load_config(config_path: &str) -> Config {
    let config_string = fs::read_to_string(config_path).expect("Failed to read config file");
    toml::from_str(&config_string).expect("Failed to parse config")
}
