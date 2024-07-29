use serde::Serialize;

use crate::{
    diagnostics::geometry_diagnostics::GeometryDiagnostics, simulation::Simulation,
    utils::config_loading::Config,
};

pub mod heat_diffusion;
pub mod heat_diffusion_rework;

#[derive(Serialize)]
struct TemperatureData {
    time: f64,
    mean_temperature: f64,
    maximum_temperature: f64,
}

pub struct HeatDiffusion {
    simulation: Simulation,
    geometry: GeometryDiagnostics,

    config: Config,
    fission_source_array: Vec<f64>,

    simulation_directions: Vec<SimulationDirections>,

    t_delta: f64,
    t_end: f64,
    time_steps: u64,

    minimum_relevant_property_index: usize,
    cell_volume: f64,

    relevant_tuples: Vec<(usize, usize, usize)>,

    material_index_array: Vec<usize>,
    temperature_array: Vec<f64>,
    temperature_array_new: Vec<f64>,

    temperature_data_array: Vec<TemperatureData>,

    source_term_constant: f64,
}

enum SimulationDirections {
    North,
    South,
    East,
    West,
    Top,
    Bottom,
}
