use chrono::{DateTime, Local};

use crate::diagnostics::BinData;
use crate::simulation::custom_runs::standard_simulation::standard_simulation;
use crate::simulation::Simulation;
use crate::utils::config_loading::load_config;
use crate::utils::data_writing::write_bin_results_vector;
use std::fs::create_dir_all;
use std::thread;

fn combine_bin_data(simulation_vector: &Vec<Simulation>) -> Vec<BinData> {
    let mut aggregated_results: Vec<BinData> = Vec::new();

    for simulation in simulation_vector {
        let bin_results = &simulation.neutron_diagnostics.neutron_position_bins;

        if aggregated_results.is_empty() {
            // If aggregated_results is empty, initialize it with a clone of the current bin_results
            aggregated_results = bin_results.clone();
        } else {
            // Add up corresponding BinData entries
            for (aggregated_bin, bin_result) in
                aggregated_results.iter_mut().zip(bin_results.iter())
            {
                aggregated_bin.add(bin_result);
            }
        }
    }

    aggregated_results
}

pub fn parallel_runs() {
    let config = load_config("config/simulation/default.toml");
    let parallelization_parameters = config.parallelization_parameters;

    let mut threads = vec![];

    // Spawn a set number of threads
    for _ in 0..parallelization_parameters.number_threads {
        // Each thread will run a set number of simulations
        threads.push(thread::spawn(move || {
            let mut results: Vec<Simulation> = Vec::new();
            for _ in 0..parallelization_parameters.simulations_per_thread {
                results.push(standard_simulation());
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

    let mut averaged_k_sum = 0.0;
    for simulation_result in &simulation_results {
        averaged_k_sum += simulation_result.neutron_diagnostics.averaged_k;
    }

    let mut averaged_power_sum = 0.0;
    for simulation_result in &simulation_results {
        averaged_power_sum += simulation_result.neutron_diagnostics.power_generated;
    }

    let combined_bins = combine_bin_data(&simulation_results);

    let local_date_time: DateTime<Local> = Local::now();
    let date_time_string = local_date_time.format("%Y-%m-%d_%H-%M-%S.%f").to_string();
    let dir_path = format!("results/diagnostics/aggregated_runs/{}", date_time_string);
    create_dir_all(&dir_path).expect("Failed to create aggregated run directory.");
    write_bin_results_vector(
        &combined_bins,
        &format!("{}/neutron_bin_results.csv", &dir_path),
    );

    let average_k = averaged_k_sum / simulation_results.len() as f64;
    println!("Average k: {:.3}", average_k);

    let average_power = averaged_power_sum / simulation_results.len() as f64;
    println!("Average power: {:.5}", average_power);
}
