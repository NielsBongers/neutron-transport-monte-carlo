use env_logger::{Builder, Env};
use log::info;

#[allow(unused)]
use nuclear::heat_diffusion::heat_diffusion::solve_fvm_from_file_data;
#[allow(unused)]
use nuclear::simulation::custom_runs::aggregate_runs::parallel_runs;
#[allow(unused)]
use nuclear::simulation::custom_runs::standard_simulation::standard_simulation;
use nuclear::utils::config_loading::load_config;

fn main() {
    Builder::from_env(Env::default().default_filter_or("info")).init();
    info!("Starting simulation.");

    let config = load_config("config/simulation/default.toml");

    if config.simulation_parameters.model_heat_diffusion {
        solve_fvm_from_file_data()
    } else {
        parallel_runs();
    }
}
