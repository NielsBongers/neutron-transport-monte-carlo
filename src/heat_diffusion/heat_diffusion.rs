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

/// This was the initial heat diffusion model code, which has since been supplanted by the new code, which is still being worked on. 
/// For now, this code will remain here as a reference, but it does no longer function. 
#[cfg(feature = "deprecated_code")]
pub fn solve_fvm(
    geometry: &GeometryDiagnostics,
    simulation_bins: &Vec<BinData>,
    halt_time: Option<f64>,
) -> bool {
    let config = load_config(Path::new("config/simulation/default.toml"));
    let heat_diffusion_parameters: crate::utils::config_loading::HeatDiffusionParametersTOML =
        config.heat_diffusion_parameters;

    let local_date_time: DateTime<Local> = Local::now();
    let date_time_string = local_date_time.format("%Y-%m-%d_%H-%M-%S.%f").to_string();
    let dir_path_string = format!("results/heat_diffusion/{}", date_time_string);
    let dir_path = Path::new(&dir_path_string);

    std::fs::create_dir_all(&dir_path).expect(
        "Failed to create path to write heat diffusion results to: results/heat_diffusion/csvs",
    );

    let element_volume = (geometry.total_length / geometry.length_count as f64)
        * (geometry.total_depth / geometry.depth_count as f64)
        * (geometry.total_height / geometry.height_count as f64);

    let halt_time = halt_time
        .expect("Halt time has to be enabled in the configuration file to use heat diffusion.");

    const ENERGY_PER_FISSION: f64 = 1.9341e+8; // eV
    const EV_TO_JOULE: f64 = 1.60218e-19; // eV/J

    let power_per_fission = ENERGY_PER_FISSION * EV_TO_JOULE / halt_time / element_volume
        * heat_diffusion_parameters.neutron_multiplier;

    let fission_source: Vec<f64> = simulation_bins
        .iter()
        .map(|bin_data| bin_data.fission_count as f64 * power_per_fission)
        .collect();

    let maximum_fission_source = fission_source
        .iter()
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or(&0.0);

    let mut time = 0.0;

    let temperature_boundary = heat_diffusion_parameters.external_temperature;
    let mut temperature: Vec<f64> =
        create_temperature_array(geometry, heat_diffusion_parameters.external_temperature);
    let mut maximum_temperature = 0.0;
    let mut simulation_index = 0;

    let dx = geometry.total_length / geometry.length_count as f64;
    let dt = heat_diffusion_parameters.time_step;

    let alpha = heat_diffusion_parameters.thermal_conductivity
        / (heat_diffusion_parameters.density * heat_diffusion_parameters.heat_capacity);

    // CFL condition calculation for the heat equation
    let cfl_number = alpha * dt / (dx * dx);
    let cfl_limit = 1.0 / (2.0 * 3 as f64);

    info!(
        "Element volume: {}\nMaximum fission source: {}\nInitial temperature: {}\ndx: {}\ndt: {}\nalpha: {}\nCFL number:{}\nCFL limit:{}",
        element_volume, maximum_fission_source, &heat_diffusion_parameters.external_temperature, dx, dt, alpha, cfl_number, cfl_limit
    );

    assert!(
        cfl_number <= cfl_limit,
        "CFL limit exceeded, reduce dt or increase dx."
    );

    let mut temperature_data = Vec::<TemperatureData>::new();

    while time <= heat_diffusion_parameters.t_end {
        let mut temperature_new = temperature.clone();

        let mut temperature_sum_for_mean = 0.0;
        let mut active_cells = 0;

        for x_bin in 1..geometry.length_count - 1 {
            for y_bin in 1..geometry.depth_count - 1 {
                for z_bin in 1..geometry.height_count - 1 {
                    let center = bins_to_index(x_bin, y_bin, z_bin, geometry);
                    if fission_source[center] == 0.0 {
                        temperature_new[center] = 293.15;
                        // Skip calculation for boundary cells
                        continue;
                    }

                    // Getting all the surrounding neighborhood cells.
                    let north = bins_to_index(x_bin, y_bin + 1, z_bin, geometry);
                    let south = bins_to_index(x_bin, y_bin - 1, z_bin, geometry);
                    let east = bins_to_index(x_bin + 1, y_bin, z_bin, geometry);
                    let west = bins_to_index(x_bin - 1, y_bin, z_bin, geometry);
                    let top = bins_to_index(x_bin, y_bin, z_bin + 1, geometry);
                    let bottom = bins_to_index(x_bin, y_bin, z_bin - 1, geometry);

                    // Array of the neighbors.
                    let neighbor_indices = [north, south, east, west, top, bottom];

                    // Current cell's temperature
                    let temperature_center = temperature[center];
                    let mut temperature_update = 0.0;

                    // Looping over all neighbors.
                    for &index in neighbor_indices.iter() {
                        if fission_source[index] == 0.0 {
                            temperature_update += dx
                                * dx
                                * heat_diffusion_parameters.convective_heat_transfer_coefficient
                                * (temperature_boundary - temperature_center)
                                / dx;
                        } else {
                            temperature_update += dx
                                * dx
                                * heat_diffusion_parameters.thermal_conductivity
                                * (temperature[index] - temperature_center)
                                / dx;
                        }
                    }

                    temperature_update += fission_source[center] * element_volume;
                    temperature_update /= (heat_diffusion_parameters.density
                        * heat_diffusion_parameters.heat_capacity)
                        * element_volume;

                    temperature_new[center] = temperature_center + temperature_update * dt;

                    temperature_sum_for_mean += temperature[center];
                    active_cells += 1;

                    if temperature_new[center] > maximum_temperature {
                        maximum_temperature = temperature_new[center];
                    }
                }
            }
        }

        let mean_temperature = temperature_sum_for_mean / active_cells as f64;

        temperature_data.push(TemperatureData {
            time,
            mean_temperature,
            maximum_temperature,
        });

        if simulation_index % 100 == 0 {
            println!(
                "At {:.4}, T_max = {:.4}, T_mean = {:.4}",
                time, maximum_temperature, mean_temperature
            );
        }

        if simulation_index % heat_diffusion_parameters.write_interval == 0 {
            write_heat_diffusion_results(geometry, &temperature, time, &dir_path);
        }

        temperature = temperature_new;
        time += dt;
        simulation_index += 1;
    }

    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("results/geometry/temperature_data.csv")
        .unwrap();

    let mut wtr = csv::Writer::from_writer(file);
    for data in temperature_data {
        wtr.serialize(data).unwrap();
    }
    wtr.flush().unwrap();

    true
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
