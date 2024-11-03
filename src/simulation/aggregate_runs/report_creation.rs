use chrono::{DateTime, Local};
use log::info;
use std::{
    fs::{create_dir_all, OpenOptions},
    io::Write,
    path::Path,
    time::Duration,
};

use crate::utils::{
    config_loading::Config,
    data_writing::{
        write_bin_results_grid, write_bin_results_vector, write_convergence_vector,
        write_fission_vector,
    },
};

use super::AggregateRunResult;

pub fn write_aggregate_report(
    config: &Config,
    aggregate_run_result: &AggregateRunResult,
    simulation_duration: &Duration,
) {
    let local_date_time: DateTime<Local> = Local::now();
    let date_time_string = local_date_time.format("%Y-%m-%d_%H-%M-%S.%f").to_string();
    let dir_path = format!(
        "results/diagnostics/runs/{} - {}",
        config.simulation_parameters.run_name, date_time_string
    );

    create_dir_all(&dir_path).expect("Failed to create aggregated run directory.");

    let bin_results_path_string = format!("{}/neutron_bin_results.csv", &dir_path);
    let fission_vector_path_string = format!("{}/neutron_fission_results.csv", &dir_path);
    let neutron_position_path_string = format!("{}/neutron_positions.csv", &dir_path);
    let convergence_per_generation_string = format!("{}/convergence.csv", &dir_path);

    write_bin_results_vector(
        &aggregate_run_result.combined_bins,
        Path::new(&bin_results_path_string),
    );
    write_fission_vector(
        &aggregate_run_result.combined_fission_vector,
        Path::new(&fission_vector_path_string),
    );
    write_bin_results_grid(
        &aggregate_run_result.bin_parameters,
        &aggregate_run_result.combined_bins,
        Path::new(&neutron_position_path_string),
    );
    write_convergence_vector(
        &aggregate_run_result.convergence_per_generation,
        Path::new(&convergence_per_generation_string),
    );

    let average_k = aggregate_run_result.averaged_k / aggregate_run_result.simulation_count as f64;
    info!("Average k: {:.3}", average_k);

    let average_power =
        aggregate_run_result.averaged_power / aggregate_run_result.simulation_count as f64;
    info!("Average power: {:.5} W", average_power);

    let mut simulation_report = OpenOptions::new()
        .create(true)
        .append(true)
        .open(format!("{}/simulation_report.dat", dir_path))
        .expect("Failed to write simulation report.");

    // Report creation
    let total_milliseconds = simulation_duration.as_millis();
    let hours = total_milliseconds / 3_600_000;
    let minutes = (total_milliseconds % 3_600_000) / 60_000;
    let seconds = (total_milliseconds % 60_000) / 1_000;
    let milliseconds = total_milliseconds % 1_000;

    let formatted_duration = format!(
        "{:02}:{:02}:{:02}.{:03}",
        hours, minutes, seconds, milliseconds
    );

    let total_fissions = aggregate_run_result.combined_fission_vector.len();

    let report_line = format!(
        "=== Simulation completed ===\n
- Settings - 
{: <30}{:>20}\n\
    {: <30}{:>20}\n\
 - Results - 
{: <30}{:>20} hh:mm:ss:ms\n\n\
    {: <30}{:>20.3}\n\
    {: <30}{:>20}\n\
    {: <30}{:>20}\n\
    {: <30}{:>20}\n\
{: <30}{:>20.3} W
",
        "Track bins:",
        config.simulation_parameters.track_bins,
        "Track fission positions:",
        config.simulation_parameters.track_fission_positions,
        "Duration:",
        formatted_duration,
        "Averaged k:",
        aggregate_run_result.averaged_k,
        "Initial neutron count:",
        config.simulation_parameters.initial_neutron_count,
        "Total neutrons:",
        aggregate_run_result.total_neutrons_tracked,
        "Total fissions:",
        total_fissions,
        "Power:",
        aggregate_run_result.averaged_power,
    );

    simulation_report
        .write(report_line.as_bytes())
        .expect("Failed to write report to file.");

    info!("\n{}", report_line);
}
