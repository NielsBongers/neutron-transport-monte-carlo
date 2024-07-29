use env_logger::{Builder, Env};
use log::info;
use nuclear::simulation::custom_runs::standard_simulation::create_simulation;
use std::path::Path;

use nuclear::diagnostics::geometry_diagnostics::GeometryDiagnostics;
use nuclear::diagnostics::plotting::plot_geometry;

use nuclear::heat_diffusion::HeatDiffusion;
use nuclear::simulation::custom_runs::aggregate_runs::parallel_runs;
use nuclear::utils::config_loading::load_config;

fn main() {
    Builder::from_env(Env::default().default_filter_or("info")).init();
    info!("Starting simulation.");

    let config = load_config(Path::new("config/simulation/default.toml"));

    if config.simulation_parameters.model_heat_diffusion {
        info!("Simulating heat diffusion.");
        let mut heat_diffusion = HeatDiffusion::new();
        heat_diffusion.solve_fvm();
    } else if config.simulation_parameters.plot_geometry {
        info!("Plotting geometry.");
        let mut simulation = create_simulation();
        simulation.components.update_cache_properties(1e6);
        let geometry = GeometryDiagnostics::new(config.geometry_plot_bins);
        plot_geometry(&mut simulation, geometry);
    } else {
        info!(
            "Running parallel simulation ({} threads, {} simulations/thread)",
            config.parallelization_parameters.number_threads,
            config.parallelization_parameters.simulations_per_thread
        );
        parallel_runs();
    };
}
