use crate::diagnostics::geometry_diagnostics::GeometryDiagnostics;
use crate::diagnostics::halt_causes::SimulationHaltCauses;
use serde::{Deserialize, Serialize};

/// Post-processing collected data.
pub mod data_post_processing;
/// Writing the diagnostics output to file and terminal.
pub mod diagnostics_output;
/// Collecting data during the simulation.
pub mod diagnostics_tracking;
/// Storing data related to geometries.
pub mod geometry_diagnostics;
/// Causes for halting the simulation.
pub mod halt_causes;
/// Plotting results for ParaView/Matplotlib.
pub mod plotting;

#[derive(Default, Clone, Serialize, Deserialize, Copy)]
pub struct BinData {
    pub neutron_count: i32,
    pub fission_count: i32,
}

#[derive(Default)]
pub struct NeutronDiagnostics {
    pub generation_number: Vec<i64>,
    pub neutron_generation_history: Vec<i64>,
    pub neutron_position_bins: Vec<BinData>,

    pub bin_parameters: GeometryDiagnostics,
    pub track_creation: bool,
    pub track_bins: bool,
    pub track_from_generation: i64,

    pub max_generation_value: i64,
    pub averaged_k: f64,
    pub halt_cause: SimulationHaltCauses,
    pub initial_neutron_count: i64,
    pub total_neutrons_tracked: i64,
    pub total_fissions: i32,
    pub power_generated: f64,
}
