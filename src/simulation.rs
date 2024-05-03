use crate::diagnostics::NeutronDiagnostics;
use crate::geometry::components::Components;
use crate::neutrons::neutron_scheduler::NeutronScheduler;

use crate::utils::config_loading::SimulationParametersTOML;

pub mod custom_runs;
pub mod initialization;
pub mod simulation;

pub struct Simulation {
    pub rng: rand::rngs::SmallRng,
    pub components: Components,
    pub neutron_scheduler: NeutronScheduler,
    pub neutron_diagnostics: NeutronDiagnostics,
    pub simulation_parameters: SimulationParametersTOML,
}
