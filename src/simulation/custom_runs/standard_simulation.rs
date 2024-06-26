use crate::diagnostics::geometry_diagnostics::GeometryDiagnostics;
use crate::diagnostics::plotting::plot_geometry;
use crate::diagnostics::NeutronDiagnostics;
use crate::geometry::components::Components;
use crate::materials::material_properties::get_material_data_vector;
use crate::neutrons::neutron_scheduler::NeutronScheduler;
use crate::simulation::Simulation;

use crate::utils::config_loading::load_config;
use crate::utils::parts_loading::load_geometries;

use rand::rngs::SmallRng;
use rand::SeedableRng;

use log::info;

pub fn standard_simulation() -> Simulation {
    // Creating the RNG object.
    let rng = SmallRng::from_entropy();

    // Loading config
    let config = load_config("config/simulation/default.toml");
    let simulation_parameters: crate::utils::config_loading::SimulationParametersTOML =
        config.simulation_parameters;
    let bin_parameters: crate::utils::config_loading::BinParametersTOML = config.bin_parameters;
    let plot_parameters: crate::utils::config_loading::PlotParametersTOML = config.plot_parameters;

    // Required structs.
    let material_data_vector = get_material_data_vector();
    let parts_vector = load_geometries(&simulation_parameters.geometries_path);
    let components: Components = Components::new(material_data_vector, parts_vector);
    components.check_material_fractions_sum();
    let neutron_scheduler: NeutronScheduler = NeutronScheduler::default();

    let bin_parameters = GeometryDiagnostics::new(
        bin_parameters.length_count,
        bin_parameters.depth_count,
        bin_parameters.height_count,
        bin_parameters.center,
        bin_parameters.total_length,
        bin_parameters.total_depth,
        bin_parameters.total_height,
    );

    let plot_parameters = GeometryDiagnostics::new(
        plot_parameters.length_count,
        plot_parameters.depth_count,
        plot_parameters.height_count,
        plot_parameters.center,
        plot_parameters.total_length,
        plot_parameters.total_depth,
        plot_parameters.total_height,
    );

    let neutron_diagnostics: NeutronDiagnostics = NeutronDiagnostics::new(
        simulation_parameters.estimate_k,
        simulation_parameters.track_bins,
        simulation_parameters.track_fission_positions,
        simulation_parameters.track_from_generation,
        bin_parameters,
        simulation_parameters.initial_neutron_count,
    );

    // Instantiating simulation.
    let mut simulation: Simulation = Simulation {
        rng,
        components,
        neutron_scheduler,
        neutron_diagnostics,
        simulation_parameters,
    };

    // Optionally plotting.
    if simulation.simulation_parameters.plot_geometry {
        plot_geometry(&mut simulation, plot_parameters);
        return simulation;
    }

    // Running the simulation and tracking time.
    use std::time::Instant;
    let now = Instant::now();
    simulation.run_simulation();

    let simulation_duration = now.elapsed();

    info!("Completed simulation - continuing with diagnostics.");

    // Diagnostics
    simulation.neutron_diagnostics.write_data(
        simulation_duration,
        simulation.simulation_parameters.halt_time,
        simulation.simulation_parameters.write_results,
    );

    let diagnostics_time = now.elapsed();
    info!("Diagnostics time: {:.3?}", diagnostics_time);

    // Returning if results are valid.
    return simulation;
}
