use crate::simulation::aggregate_runs::standard_simulation::standard_simulation;
use crate::simulation::Simulation;
use crate::utils::config_loading::load_config;
use std::path::Path;
use std::thread;

use super::post_processing::post_process_aggregate_runs;

pub fn parallel_runs() {
    let config = load_config(Path::new("config/simulation/default.toml"));

    let mut threads = vec![];

    let number_threads = config.parallelization_parameters.number_threads;
    let simulations_per_thread = config
        .parallelization_parameters
        .simulations_per_thread
        .clone();

    let maximum_simulation_index = number_threads * simulations_per_thread;

    use std::time::Instant;
    let now = Instant::now();

    // Spawn a set number of threads
    for thread_index in 0..number_threads {
        // Each thread will run a set number of simulations
        threads.push(thread::spawn(move || {
            let mut results: Vec<Simulation> = Vec::new();
            for simulation_index in 0..simulations_per_thread {
                let simulation_index = (1 + thread_index) * (1 + simulation_index);

                results.push(standard_simulation(
                    simulation_index,
                    maximum_simulation_index,
                ));
            }
            results
        }));
    }

    // Collect results from all threads
    let mut simulation_results = Vec::new();
    for thread in threads {
        match thread.join() {
            Ok(result) => simulation_results.extend(result),
            Err(_) => eprintln!("Thread failed to complete"),
        }
    }

    let simulation_duration = now.elapsed();

    // Post-process our results
    post_process_aggregate_runs(&config, simulation_results, &simulation_duration);
}
