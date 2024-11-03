use crate::simulation::aggregate_runs::report_creation::write_aggregate_report;
use crate::simulation::aggregate_runs::AggregateRunResult;
use crate::utils::config_loading::Config;
use crate::{diagnostics::BinData, simulation::Simulation, utils::vectors::Vec3D};

use std::collections::HashMap;
use std::time::Duration;

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

pub fn post_process_aggregate_runs(
    config: &Config,
    simulation_results: Vec<Simulation>,
    simulation_time: &Duration,
) {
    // General analysis
    let simulation_count = simulation_results.len();

    let mut averaged_k = 0.0;
    let mut averaged_power = 0.0;
    let mut total_neutrons_tracked = 0;
    for simulation_result in &simulation_results {
        averaged_k += simulation_result.neutron_diagnostics.averaged_k;
        averaged_power += simulation_result.neutron_diagnostics.power_generated;
        total_neutrons_tracked += simulation_result.neutron_diagnostics.total_neutrons_tracked;
    }

    averaged_k /= simulation_count as f64;
    averaged_power /= simulation_count as f64;

    // Convergence analysis
    let mut intermediate_convergence_per_generation: HashMap<i64, (f64, usize)> = HashMap::new();
    for simulation_result in &simulation_results {
        for &(generation, convergence) in
            &simulation_result.neutron_diagnostics.convergence_tracking
        {
            let entry = intermediate_convergence_per_generation
                .entry(generation)
                .or_insert((0.0, 0));
            entry.0 += convergence;
            entry.1 += 1;
        }
    }

    let mut convergence_per_generation: Vec<(i64, f64)> = intermediate_convergence_per_generation
        .iter()
        .map(|(&generation, &(sum_convergence, count))| {
            (generation, sum_convergence / count as f64)
        })
        .collect();

    convergence_per_generation.sort_by_key(|&(generation, _)| generation);

    let combined_bins: Vec<BinData> = combine_bin_data(&simulation_results);
    let combined_fission_vector: Vec<Vec3D> = combine_fission_vector_data(&simulation_results);
    let bin_parameters = simulation_results[0]
        .neutron_diagnostics
        .bin_parameters
        .clone();

    let aggregate_run_result = AggregateRunResult {
        simulation_count,
        combined_bins,
        combined_fission_vector,
        averaged_k,
        averaged_power,
        total_neutrons_tracked,
        bin_parameters,
        convergence_per_generation,
    };

    write_aggregate_report(&config, &aggregate_run_result, simulation_time);
}
