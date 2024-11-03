use crate::utils::vectors::Vec3D;
use serde::Deserialize;
use std::fs;
use std::path::Path;
use toml;

/// Struct containing the three parameters we can load in. Used by serde and returned to the main function.
#[derive(Deserialize)]
pub struct Config {
    pub parallelization_parameters: ParallelizationParametersTOML,
    pub simulation_parameters: SimulationParametersTOML,
    pub heat_diffusion_parameters: HeatDiffusionParametersTOML,
    pub neutron_bins: GridBinParametersTOML,
    pub geometry_plot_bins: GridBinParametersTOML,
    pub heat_diffusion_bins: GridBinParametersTOML,
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
    pub run_name: String,
    pub neutron_initial_position: Vec3D,
    pub neutron_generation_cap: i64,
    pub neutron_count_cap: i64,
    pub initial_neutron_count: i64,
    pub variance_reduction: bool,
    pub specified_neutron_count: i64,
    pub neutron_fission_count_cap: i64,
    pub estimate_k: bool,
    pub track_fission_positions: bool,
    pub track_bins: bool,
    pub track_from_generation: i64,
    pub plot_geometry: bool,
    pub halt_time: Option<f64>,
    pub maximum_neutron_energy_difference: f64,
    pub geometries_path: String,
    pub model_heat_diffusion: bool,
    pub calculate_convergence: bool,
    pub convergence_analysis_period: i64,
    pub minimum_convergence_level: f64,
}

/// Parameters for heat diffusion modelling.
#[derive(Deserialize)]
pub struct HeatDiffusionParametersTOML {
    pub source_data_file: String,
    pub minimum_relevant_property_index: usize,
    pub neutron_multiplier: f64,
    pub initial_internal_temperature: f64,
    pub external_temperature: f64,
    pub t_delta: f64,
    pub t_end: f64,
    pub write_interval: i64,
    pub convective_heat_transfer_coefficient: f64,
    pub save_files: bool,
}

/// Parameters for the bins for neutron behavior, plotting, and heat diffusion.
#[derive(Deserialize, Copy, Clone)]
pub struct GridBinParametersTOML {
    pub length_count: usize,
    pub depth_count: usize,
    pub height_count: usize,
    pub total_length: f64,
    pub total_depth: f64,
    pub total_height: f64,
    pub center: Vec3D,
}

/// Loading the config file into a ```Config``` object and returning that.
pub fn load_config(config_path: &Path) -> Config {
    let config_string = fs::read_to_string(config_path).expect("Failed to read config file");
    toml::from_str(&config_string).expect("Failed to parse config")
}
