use crate::diagnostics::geometry_diagnostics::GeometryDiagnostics;
use crate::heat_diffusion::SimulationDirections;
use crate::simulation::custom_runs::standard_simulation::create_simulation;
use crate::utils::config_loading::load_config;
use crate::utils::config_loading::GridBinParametersTOML;
use crate::utils::data_loading::load_fission_vector;
use crate::utils::vectors::Vec3D;

use csv::WriterBuilder;
use indicatif::ProgressBar;
use indicatif::ProgressStyle;

#[allow(unused)]
use log::{error, info, warn};

use core::f64;
use std::fs;
use std::fs::OpenOptions;
use std::io::BufWriter;
use std::path::Path;
use std::process::Command;
use std::thread;

use crate::heat_diffusion::HeatDiffusion;
use crate::heat_diffusion::TemperatureData;

impl SimulationDirections {
    fn all_directions() -> Vec<SimulationDirections> {
        vec![
            SimulationDirections::North,
            SimulationDirections::South,
            SimulationDirections::East,
            SimulationDirections::West,
            SimulationDirections::Top,
            SimulationDirections::Bottom,
        ]
    }
}

impl TemperatureData {
    fn new(time: f64, mean_temperature: f64, maximum_temperature: f64) -> Self {
        TemperatureData {
            time,
            mean_temperature,
            maximum_temperature,
        }
    }
}

impl HeatDiffusion {
    pub fn new() -> HeatDiffusion {
        let config = load_config(Path::new("config/simulation/default.toml"));

        let heat_diffusion_bins: GridBinParametersTOML = config.heat_diffusion_bins;
        let geometry = GeometryDiagnostics::new(heat_diffusion_bins);

        let material_index_array = Vec::<usize>::new();

        let fission_source_array = Vec::<f64>::new();

        let temperature_array = Vec::<f64>::new();
        let temperature_array_new = Vec::<f64>::new();

        let temperature_data_array = Vec::<TemperatureData>::new();

        let relevant_tuples = Vec::<(usize, usize, usize)>::new();

        let cell_volume = geometry.delta_x * geometry.delta_y * geometry.delta_z;

        let t_delta = config.heat_diffusion_parameters.t_delta;
        let t_end = config.heat_diffusion_parameters.t_end;
        let time_steps = (config.heat_diffusion_parameters.t_end
            / config.heat_diffusion_parameters.t_delta) as u64;

        let halt_time = config
            .simulation_parameters
            .halt_time
            .expect("Halt time has to be specified for heat diffusion simulations.");

        let minimum_relevant_property_index = config
            .heat_diffusion_parameters
            .minimum_relevant_property_index;

        let simulation_directions = SimulationDirections::all_directions();
        let simulation = create_simulation();

        // We have the source term here as J/bin over the entire simulation.
        // We need to convert this to J/m3/s to act as a source term in FVM, so we convert it to that, and apply a multiplier that scales the distribution.
        let source_term_constant = 1.0 / cell_volume * 1.0 / halt_time
            * config.heat_diffusion_parameters.neutron_multiplier;

        HeatDiffusion {
            simulation,
            geometry,
            config,
            fission_source_array,
            simulation_directions,
            t_delta,
            t_end,
            time_steps,
            minimum_relevant_property_index,
            cell_volume,
            material_index_array,
            temperature_array,
            temperature_array_new,
            source_term_constant,
            relevant_tuples,
            temperature_data_array,
        }
    }

    fn create_grid_array<T: Default + Clone>(&self) -> Vec<T> {
        vec![
            T::default();
            (self.geometry.length_count + 1)
                * (self.geometry.depth_count + 1)
                * (self.geometry.height_count + 1)
        ]
    }

    fn create_property_array(&mut self) {
        for x_bin in 1..self.geometry.length_count - 1 {
            for y_bin in 1..self.geometry.depth_count - 1 {
                for z_bin in 1..self.geometry.height_count - 1 {
                    let center_index = self.geometry.bins_to_index(&x_bin, &y_bin, &z_bin);

                    let (x, y, z): (f64, f64, f64) =
                        self.geometry.index_to_coordinates(x_bin, y_bin, z_bin);

                    let neutron_position = Vec3D { x, y, z };

                    let (current_material_index, _) = self
                        .simulation
                        .components
                        .get_material_index(&mut self.simulation.rng, &neutron_position);

                    self.material_index_array[center_index] = current_material_index;
                }
            }
        }
    }

    fn calculate_interface_flux(
        &self,
        direction: &SimulationDirections,
        (x_bin, y_bin, z_bin): (&usize, &usize, &usize),
    ) -> f64 {
        let center_index = self.geometry.bins_to_index(x_bin, y_bin, z_bin);

        let (index, interface_area, node_distance): (usize, f64, f64) = match direction {
            SimulationDirections::North => {
                let north_index = self.geometry.bins_to_index(x_bin, &(y_bin + 1), z_bin);
                let interface_area = self.geometry.delta_x * self.geometry.delta_z;
                (north_index, interface_area, self.geometry.delta_y)
            }
            SimulationDirections::South => {
                let south_index = self.geometry.bins_to_index(x_bin, &(y_bin - 1), z_bin);
                let interface_area = self.geometry.delta_x * self.geometry.delta_z;
                (south_index, interface_area, self.geometry.delta_y)
            }
            SimulationDirections::East => {
                let east_index = self.geometry.bins_to_index(&(x_bin + 1), y_bin, z_bin);
                let interface_area = self.geometry.delta_y * self.geometry.delta_z;
                (east_index, interface_area, self.geometry.delta_x)
            }
            SimulationDirections::West => {
                let west_index = self.geometry.bins_to_index(&(x_bin - 1), y_bin, z_bin);
                let interface_area = self.geometry.delta_y * self.geometry.delta_z;
                (west_index, interface_area, self.geometry.delta_y)
            }
            SimulationDirections::Top => {
                let top_index = self.geometry.bins_to_index(x_bin, y_bin, &(z_bin + 1));
                let interface_area = self.geometry.delta_x * self.geometry.delta_y;
                (top_index, interface_area, self.geometry.delta_z)
            }
            SimulationDirections::Bottom => {
                let bottom_index = self.geometry.bins_to_index(x_bin, y_bin, &(z_bin - 1));
                let interface_area = self.geometry.delta_x * self.geometry.delta_y;
                (bottom_index, interface_area, self.geometry.delta_z)
            }
        };

        let temperature_center = self.temperature_array[center_index];
        let temperature_adjacent = self.temperature_array[index];

        let property_index_center = self.material_index_array[center_index];
        let property_index_adjacent = self.material_index_array[index];

        let material_center =
            &self.simulation.components.material_data_vector[property_index_center];
        let material_adjacent =
            &self.simulation.components.material_data_vector[property_index_adjacent];

        // Checking whether we have an internal node or a boundary condition.
        if property_index_adjacent >= self.minimum_relevant_property_index {
            let averaged_thermal_conductivity = (material_center.thermal_conductivity
                + material_adjacent.thermal_conductivity)
                / 2.0;

            let heat_flux = averaged_thermal_conductivity
                * interface_area
                * (temperature_adjacent - temperature_center)
                / node_distance;

            heat_flux
        } else {
            let heat_transfer_coefficient = self
                .config
                .heat_diffusion_parameters
                .convective_heat_transfer_coefficient;

            let heat_flux = heat_transfer_coefficient
                * interface_area
                * (self.config.heat_diffusion_parameters.external_temperature - temperature_center)
                / node_distance;

            heat_flux
        }
    }

    fn write_fvm_output(
        temperature_array: &Vec<f64>,
        geometry: &GeometryDiagnostics,
        _material_index_array: &Vec<usize>,
        _minimum_relevant_property_index: usize,
        time: f64,
        _relevant_tuples: Vec<(usize, usize, usize)>,
    ) {
        let dir_path = Path::new("results/heat_diffusion/csvs");
        let file_name = format!("{:.5}.csv", time);
        let file_path = dir_path.join(&file_name);

        fs::create_dir_all(&dir_path).expect("Failed to create temperature folder");

        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(file_path)
            .expect("Failed to open temperatures file.");

        let buf_writer = BufWriter::new(file);
        let mut wtr = WriterBuilder::new()
            .has_headers(false)
            .from_writer(buf_writer);

        // Write the header
        wtr.write_record(&["x", "y", "z", "T"])
            .expect("Failed to write temperature headers.");

        for x_bin in 1..geometry.length_count - 1 {
            for y_bin in 1..geometry.depth_count - 1 {
                for z_bin in 1..geometry.height_count - 1 {
                    let center = geometry.bins_to_index(&x_bin, &y_bin, &z_bin);
                    let temperature_value = temperature_array[center];
                    let (x, y, z) = geometry.index_to_coordinates(x_bin, y_bin, z_bin);
                    wtr.write_record(&[
                        x.to_string(),
                        y.to_string(),
                        z.to_string(),
                        temperature_value.to_string(),
                    ])
                    .expect("Failed to write record.");
                }
            }
        }

        wtr.flush().expect("Failed to flush buffer to file.");
    }

    pub fn load_fission_source(&self, fission_source_file_path: &Path) -> Vec<f64> {
        let fission_event_vector = load_fission_vector(fission_source_file_path);
        let mut fission_source_array: Vec<f64> = self.create_grid_array();

        const ENERGY_PER_FISSION: f64 = 1.9341e+8; // eV
        const EV_TO_JOULE: f64 = 1.60218e-19; // eV/J

        fission_event_vector
            .iter()
            .filter_map(|&neutron_position| self.geometry.get_current_bin(neutron_position))
            .for_each(|fission_bin_index| {
                fission_source_array[fission_bin_index] += ENERGY_PER_FISSION * EV_TO_JOULE
            });

        fission_source_array
    }

    pub fn write_temperature_history(&self) {
        let dir_path = Path::new("results/heat_diffusion");
        let file_path = dir_path.join("temperature_data.csv");

        fs::create_dir_all(&dir_path).expect("Failed to create temperature folder");

        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&file_path)
            .expect("Failed to open temperatures file.");

        let buf_writer = BufWriter::new(file);
        let mut wtr = WriterBuilder::new()
            .has_headers(true)
            .from_writer(buf_writer);

        for record in &self.temperature_data_array {
            wtr.serialize(record)
                .expect("Failed to serialize temperature data");
        }

        wtr.flush().expect("Failed to flush CSV.");
    }

    pub fn initialize_fvm(&mut self) {
        let fission_source_file_path =
            Path::new(&self.config.heat_diffusion_parameters.source_data_file);

        self.fission_source_array = self.load_fission_source(fission_source_file_path);

        match fs::remove_dir_all("results/heat_diffusion") {
            Err(_) => warn!("Failed to delete results directory."),
            _ => (),
        };

        self.simulation.components.update_cache_properties(1e6);

        self.material_index_array = self.create_grid_array();
        self.create_property_array();

        self.temperature_array = self.create_grid_array();
        self.temperature_array = self
            .temperature_array
            .iter()
            .zip(self.material_index_array.iter())
            .map(|(&_, &material_index)| {
                if material_index >= self.minimum_relevant_property_index {
                    self.config
                        .heat_diffusion_parameters
                        .initial_internal_temperature
                } else {
                    self.config.heat_diffusion_parameters.external_temperature
                }
            })
            .collect();
        self.temperature_array_new = self.temperature_array.clone();

        for x_bin in 1..self.geometry.length_count - 1 {
            for y_bin in 1..self.geometry.depth_count - 1 {
                for z_bin in 1..self.geometry.height_count - 1 {
                    let center_index = self.geometry.bins_to_index(&x_bin, &y_bin, &z_bin);
                    let center_material_index = self.material_index_array[center_index];

                    if center_material_index >= self.minimum_relevant_property_index {
                        self.relevant_tuples.push((x_bin, y_bin, z_bin));
                    }
                }
            }
        }
    }

    pub fn solve_fvm(&mut self) -> () {
        self.initialize_fvm();

        let mut time = 0.0;
        let mut time_index = 0;

        let mut file_writing_handles: Vec<std::thread::JoinHandle<()>> = vec![];

        let pb = ProgressBar::new(self.time_steps);
        pb.set_style(ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos}/{len} ({eta}) {percent}%")
            .expect("Error in setting progress bar template")
            );

        pb.set_message("Processing...");

        while time < self.t_end {
            let mut mean_temperature = 0.0;
            let mut maximum_temperature = f64::MIN;

            for (x_bin, y_bin, z_bin) in &self.relevant_tuples {
                let center_index = self.geometry.bins_to_index(x_bin, y_bin, z_bin);
                let center_material_index = self.material_index_array[center_index];

                if center_material_index < self.minimum_relevant_property_index {
                    continue;
                }

                let center_material_data =
                    &self.simulation.components.material_data_vector[center_material_index];
                let temperature_center = self.temperature_array[center_index];

                // Some diagnostics.
                mean_temperature += temperature_center;
                if temperature_center > maximum_temperature {
                    maximum_temperature = temperature_center;
                }

                // It's a bit complicated to calculate CFL: we have prescribed heat fluxes, so it's possible that cells get a negative temperature.
                // For now, this is an easy way to stop the code from running, rather than waiting and seeing it failed from the result files.
                if temperature_center.is_nan() {
                    panic!(
                        "Encountered NaN in temperature_center at t={} ({}/{}), halting simulation.",
                        time, time_index, self.time_steps
                    );
                }

                let center_heat_flux: f64 = self
                    .simulation_directions
                    .iter()
                    .map(|direction: &SimulationDirections| {
                        self.calculate_interface_flux(&direction, (x_bin, y_bin, z_bin))
                    })
                    .sum();

                let source_term =
                    self.source_term_constant * self.fission_source_array[center_index];

                let inertial_term = center_material_data.density
                    * center_material_data.heat_capacity
                    * self.cell_volume;

                #[allow(non_snake_case)]
                let dT_dt = (center_heat_flux + source_term * self.cell_volume) / inertial_term;

                self.temperature_array_new[center_index] =
                    temperature_center + dT_dt * self.t_delta;
            }

            if time_index % self.config.heat_diffusion_parameters.write_interval == 0
                && self.config.heat_diffusion_parameters.save_files
            {
                let temp_array_clone = self.temperature_array.clone().to_owned();
                let geometry_clone = self.geometry.clone().to_owned();
                let material_index_array_clone = self.material_index_array.clone();
                let minimum_relevant_property_index = self.minimum_relevant_property_index;
                let relevant_tuples = self.relevant_tuples.clone();

                let current_time = time_index as f64 * self.t_delta;

                let handle = thread::spawn(move || {
                    HeatDiffusion::write_fvm_output(
                        &temp_array_clone,
                        &geometry_clone,
                        &material_index_array_clone,
                        minimum_relevant_property_index,
                        current_time,
                        relevant_tuples,
                    );
                });

                file_writing_handles.push(handle);
            }

            // Pure magic: it shaves 20s of the previously 60s simulation!
            std::mem::swap(&mut self.temperature_array, &mut self.temperature_array_new);

            mean_temperature = mean_temperature / self.relevant_tuples.len() as f64;

            let temperature_data =
                TemperatureData::new(time, mean_temperature, maximum_temperature);

            self.temperature_data_array.push(temperature_data);

            time += self.config.heat_diffusion_parameters.t_delta;
            time_index += 1;
            pb.inc(1);
        }

        self.write_temperature_history();

        for handle in file_writing_handles {
            handle.join().expect("Thread panicked");
        }

        info!("Converting CSVs to VTKs.");

        let python_csv_vtk_mode = "heat_diffusion";
        Command::new("python3")
            .arg("scripts/post_processing/csv_to_vtk.py")
            .arg(python_csv_vtk_mode)
            .output()
            .expect("Failed to execute script");
    }
}
