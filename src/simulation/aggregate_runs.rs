use crate::{
    diagnostics::{geometry_diagnostics::GeometryDiagnostics, BinData},
    utils::vectors::Vec3D,
};

pub mod aggregate_runs;
pub mod post_processing;
pub mod report_creation;
pub mod standard_simulation;

pub struct AggregateRunResult {
    simulation_count: usize,

    combined_bins: Vec<BinData>,
    combined_fission_vector: Vec<Vec3D>,

    total_neutrons_tracked: i64,

    averaged_k: f64,
    averaged_power: f64,

    bin_parameters: GeometryDiagnostics,
    convergence_per_generation: Vec<(i64, f64)>,
}
