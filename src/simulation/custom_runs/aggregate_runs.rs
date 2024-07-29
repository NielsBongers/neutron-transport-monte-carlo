use chrono::{DateTime, Local};

use crate::diagnostics::BinData;
use crate::simulation::custom_runs::standard_simulation::standard_simulation;
use crate::simulation::Simulation;
use crate::utils::config_loading::load_config;
use crate::utils::data_writing::{write_bin_results_vector, write_fission_vector};
use crate::utils::vectors::Vec3D;
use log::info;
use std::fs::create_dir_all;
use std::path::Path;
use std::thread;

fn combine_bin_data(simulation_results: &Vec<Simulation>) -> Vec<BinData> {
    let mut aggregated_results: Vec<BinData> = Vec::new();

    for simulation in simulation_results {
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

fn combine_fission_vector_data(simulation_results: &Vec<Simulation>) -> Vec<Vec3D> {
    let mut aggregated_fission_vector: Vec<Vec3D> = Vec::new();

    for simulation in simulation_results {
        let fission_vector = &simulation.neutron_diagnostics.neutron_fission_locations;
        aggregated_fission_vector.extend(fission_vector);
    }

    aggregated_fission_vector
}

pub fn parallel_runs() {
    let config = load_config(Path::new("config/simulation/default.toml"));

    let mut threads = vec![];

    let number_threads = config.parallelization_parameters.number_threads;
    let simulations_per_thread = config
        .parallelization_parameters
        .simulations_per_thread
        .clone();

    let maximum_simulation_index = number_threads * simulations_per_thread;

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

    let mut averaged_k_sum = 0.0;
    for simulation_result in &simulation_results {
        averaged_k_sum += simulation_result.neutron_diagnostics.averaged_k;
    }

    let mut averaged_power_sum = 0.0;
    for simulation_result in &simulation_results {
        averaged_power_sum += simulation_result.neutron_diagnostics.power_generated;
    }

    let combined_bins = combine_bin_data(&simulation_results);
    let combined_fission_vector = combine_fission_vector_data(&simulation_results);

    let local_date_time: DateTime<Local> = Local::now();
    let date_time_string = local_date_time.format("%Y-%m-%d_%H-%M-%S.%f").to_string();
    let dir_path = format!(
        "results/diagnostics/aggregated_runs/{} - {}",
        config.simulation_parameters.run_name, date_time_string
    );

    create_dir_all(&dir_path).expect("Failed to create aggregated run directory.");

    let bin_results_path_string = format!("{}/neutron_bin_results.csv", &dir_path);
    let fission_vector_path_string = format!("{}/neutron_fission_results.csv", &dir_path);

    write_bin_results_vector(&combined_bins, Path::new(&bin_results_path_string));
    write_fission_vector(
        combined_fission_vector,
        Path::new(&fission_vector_path_string),
    );

    let average_k = averaged_k_sum / simulation_results.len() as f64;
    info!("Average k: {:.3}", average_k);

    let average_power = averaged_power_sum / simulation_results.len() as f64;
    info!("Average power: {:.5} W", average_power);
}
