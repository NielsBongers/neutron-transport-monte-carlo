#![allow(unused)]

use crate::diagnostics::geometry_diagnostics::GeometryDiagnostics;
use crate::diagnostics::BinData;
use crate::utils::config_loading::load_config;
use crate::utils::data_loading::load_bin_data_vector;
use crate::utils::data_writing::write_bin_results_grid;
use chrono::{DateTime, Local};
use log::info;
use serde::Serialize;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

#[derive(Serialize)]
struct TemperatureData {
    time: f64,
    mean_temperature: f64,
    maximum_temperature: f64,
}

pub fn bins_to_index(
    x_bin: usize,
    y_bin: usize,
    z_bin: usize,
    geometry: &GeometryDiagnostics,
) -> usize {
    x_bin + y_bin * geometry.length_count + z_bin * geometry.length_count * geometry.depth_count
}

pub fn write_heat_diffusion_results(
    geometry: &GeometryDiagnostics,
    temperature: &Vec<f64>,
    time: f64,
    dir_path: &Path,
) {
    let mut temperature_file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(format!("{}/{:.5}.csv", &dir_path.display(), time))
        .expect("Failed to open temperatures file.");

    temperature_file
        .write("x,y,z,T\n".as_bytes())
        .expect("Failed to write temperature headers.");

    for x_bin in 1..geometry.length_count - 1 {
        for y_bin in 1..geometry.depth_count - 1 {
            for z_bin in 1..geometry.height_count - 1 {
                let center = bins_to_index(x_bin, y_bin, z_bin, geometry);

                let x = geometry.x_min
                    + (x_bin as f64 / geometry.length_count as f64) * geometry.total_length;
                let y = geometry.y_min
                    + (y_bin as f64 / geometry.depth_count as f64) * geometry.total_depth;
                let z = geometry.z_min
                    + (z_bin as f64 / geometry.height_count as f64) * geometry.total_height;

                let temperature_value = temperature[center];

                let write_string = format!("{},{},{},{}\n", x, y, z, temperature_value);

                temperature_file.write(write_string.as_bytes()).unwrap();
            }
        }
    }
}

pub fn create_temperature_array(geometry: &GeometryDiagnostics, default_value: f64) -> Vec<f64> {
    vec![
        default_value;
        (geometry.length_count + 1) * (geometry.depth_count + 1) * (geometry.height_count + 1)
    ]
}

pub fn solve_fvm_from_file_data() {
    let config = load_config(Path::new("config/simulation/default.toml"));
    let heat_diffusion_parameters = config.heat_diffusion_parameters;
    let grid_bin_parameters = config.neutron_bins;

    let geometry = GeometryDiagnostics::new(grid_bin_parameters);

    let simulation_bins =
        load_bin_data_vector(Path::new(&heat_diffusion_parameters.source_data_file));

    write_bin_results_grid(
        &geometry,
        &simulation_bins,
        Path::new(r"D:\Desktop\nuclear-rust\results\geometry\neutron_bins.csv"),
    );

    // solve_fvm(
    //     &geometry,
    //     &simulation_bins,
    //     config.simulation_parameters.halt_time,
    // );
}
