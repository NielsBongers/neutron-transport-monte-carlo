use crate::diagnostics::geometry_diagnostics::GeometryDiagnostics;
use crate::diagnostics::NeutronDiagnostics;
use crate::geometry::components::Components;
use crate::materials::material_properties::get_material_data_vector;
use crate::neutrons::neutron_scheduler::NeutronScheduler;
use crate::simulation::Simulation;
use crate::utils::config_loading::GridBinParametersTOML;

use crate::utils::config_loading::load_config;
use crate::utils::parts_loading::load_geometries;

use rand::rngs::SmallRng;
use rand::SeedableRng;
use std::path::Path;

use log::info;

pub fn create_simulation() -> Simulation {
    // Creating the RNG object.
    let rng = SmallRng::from_entropy();

    // Loading config
    let config = load_config(Path::new("config/simulation/default.toml"));
    let simulation_parameters: crate::utils::config_loading::SimulationParametersTOML =
        config.simulation_parameters;
    let neutron_bin_parameters: GridBinParametersTOML = config.neutron_bins;

    // Required structs.
    let material_data_vector = get_material_data_vector();
    let parts_vector = load_geometries(Path::new(&simulation_parameters.geometries_path));
    let components: Components = Components::new(material_data_vector, parts_vector);
    components.check_material_fractions_sum();
    let neutron_scheduler: NeutronScheduler = NeutronScheduler::default();

    let bin_parameters = GeometryDiagnostics::new(neutron_bin_parameters);

    let neutron_diagnostics: NeutronDiagnostics = NeutronDiagnostics::new(
        simulation_parameters.estimate_k,
        simulation_parameters.track_bins,
        simulation_parameters.track_fission_positions,
        simulation_parameters.track_from_generation,
        bin_parameters,
        simulation_parameters.initial_neutron_count,
    );

    // Instantiating simulation.
    let simulation: Simulation = Simulation {
        rng,
        components,
        neutron_scheduler,
        neutron_diagnostics,
        simulation_parameters,
    };

    simulation
}

pub fn standard_simulation(simulation_index: i64, maximum_simulation_index: i64) -> Simulation {
    let mut simulation = create_simulation();

    // Running the simulation.
    simulation.run_simulation();

    info!(
        "Completed simulation {}/{}.",
        simulation_index, maximum_simulation_index
    );

    // Diagnostics
    simulation
        .neutron_diagnostics
        .post_process(simulation.simulation_parameters.halt_time);

    // Returning if results are valid.
    return simulation;
}
