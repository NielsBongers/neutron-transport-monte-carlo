use crate::diagnostics::NeutronDiagnostics;
use crate::utils::data_writing::write_bin_results_grid;
use chrono::{DateTime, Local};
use log::{debug, error, info};
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::time::Duration;

impl NeutronDiagnostics {
    pub fn write_simulation_report(
        &mut self,
        dir_path: &str,
        simulation_duration: Duration,
        halt_time: Option<f64>,
    ) {
        let mut simulation_report = OpenOptions::new()
            .create(true)
            .append(true)
            .open(format!("{}/simulation_report.dat", dir_path))
            .expect("Failed to write simulation report.");

        let total_milliseconds = simulation_duration.as_millis();
        let hours = total_milliseconds / 3_600_000;
        let minutes = (total_milliseconds % 3_600_000) / 60_000;
        let seconds = (total_milliseconds % 60_000) / 1_000;
        let milliseconds = total_milliseconds % 1_000;

        let formatted_duration = format!(
            "{:02}:{:02}:{:02}.{:03}",
            hours, minutes, seconds, milliseconds
        );

        let energy_per_fission = 1.9341e+8; // eV
        let ev_to_joule = 1.60218e-19;
        let total_energy = self.total_fissions as f64 * energy_per_fission * ev_to_joule; // J produced

        if let Some(halt_time) = halt_time {
            self.power_generated = total_energy / halt_time;
        } else {
            self.power_generated = 0.0;
        }

        let mut power_data = OpenOptions::new()
            .create(true)
            .append(true)
            .open(format!("{}/power_data.csv", dir_path))
            .expect("Creating file to write diagnostics results to.");

        let power_results = format!(
            "{:?},{:?},{:?},{:?}\n",
            self.total_fissions, total_energy, self.power_generated, halt_time
        );

        power_data.write(power_results.as_bytes()).unwrap();

        let report_line = format!(
            "=== Simulation completed ===\n
 - Results - 
{: <30}{:>20} hh:mm:ss:ms\n\
{: <30}{:>20}\n\
{: <30}{:>20.3}\n\
{: <30}{:>20}\n\
{: <30}{:>20}\n\
{: <30}{:>20}\n\
{: <30}{:>20.9} J\n\
{: <30}{:>20.3} W\n\
{}{}\n
- Settings - 
{: <30}{:>20}\n\
{: <30}{:>20}\n\
{: <30}{:>20}\n\
{: <30}{:>20}\n\
",
            "Duration:",
            formatted_duration,
            "Maximum generation value:",
            self.max_generation_value,
            "Averaged k:",
            self.averaged_k,
            "Initial neutron count:",
            self.initial_neutron_count,
            "Total neutrons:",
            self.total_neutrons_tracked,
            "Total fissions:",
            self.total_fissions,
            "Total energy produced:",
            total_energy,
            "Power:",
            self.power_generated,
            "Halt cause: ",
            self.halt_cause,
            "Track creation:",
            self.track_creation,
            "Track positions:",
            self.track_positions,
            "Track energies:",
            self.track_energies,
            "Track bins:",
            self.track_bins,
        );

        simulation_report
            .write(report_line.as_bytes())
            .expect("Failed to write report to file.");

        info!("\n{}", report_line);
    }

    pub fn write_data(
        &mut self,
        simulation_duration: Duration,
        halt_time: Option<f64>,
        write_results: bool,
    ) {
        let local_date_time: DateTime<Local> = Local::now();
        let date_time_string = local_date_time.format("%Y-%m-%d_%H-%M-%S.%f").to_string();
        let dir_path = format!("results/diagnostics/individual_runs/{}", date_time_string);

        match fs::create_dir_all(&dir_path) {
            Err(why) => error!("! {:?}", why.kind()),
            Ok(_) => debug!("Successfully created directory {}", dir_path),
        }

        self.estimate_k();

        self.total_fissions = self
            .neutron_position_bins
            .iter()
            .map(|bin_data| bin_data.fission_count)
            .sum();

        if write_results {
            if !self.creation_times.is_empty() {
                let mut creation_file = OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(format!("{}/creation_results.csv", dir_path))
                    .expect("Creating file to write diagnostics results to.");

                creation_file
                    .write("creation_time,generation_number\n".as_bytes())
                    .expect("Writing diagnostics headers.");

                for (creation_time, generation_number) in self
                    .creation_times
                    .iter()
                    .zip(self.generation_number.iter())
                {
                    let write_string = format!("{},{}\n", creation_time, generation_number);

                    creation_file
                        .write(write_string.as_bytes())
                        .expect("Writing diagnostics to file.");
                }
            }

            if !self.neutron_positions.is_empty() {
                let mut position_file = OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(format!("{}/position_results.csv", dir_path))
                    .expect("Opening neutron position data file.");

                position_file
                    .write("x,y,z\n".as_bytes())
                    .expect("Writing neutron position data headers.");

                for neutron_position in self.neutron_positions.iter() {
                    let write_string = format!(
                        "{:.5},{:.5},{:.5}\n",
                        neutron_position.x, neutron_position.y, neutron_position.z
                    );

                    position_file
                        .write(write_string.as_bytes())
                        .expect("Writing neutron position to file");
                }
            }

            if !self.generation_counts.is_empty() {
                let mut generation_counts_file = OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(format!("{}/generation_counts.csv", dir_path))
                    .expect("Opening generation counts file.");

                generation_counts_file
                    .write("generation,generation_counts\n".as_bytes())
                    .expect("Writing generation counts headers.");

                for (generation, generation_count) in self.generation_counts.iter().enumerate() {
                    let write_string = format!("{},{}\n", generation, generation_count,);

                    generation_counts_file
                        .write(write_string.as_bytes())
                        .expect("Writing generation count to file.");
                }
            }

            if !self.neutron_energies.is_empty() {
                let mut neutron_energies_file = OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(format!("{}/neutron_energies.csv", dir_path))
                    .expect("Opening neutron energies file.");

                neutron_energies_file
                    .write("iteration,timestamp,neutron_energy\n".as_bytes())
                    .expect("Writing neutron energies headers.");

                for (generation, (timestamp, neutron_energy)) in
                    self.neutron_energies.iter().enumerate()
                {
                    let write_string =
                        format!("{},{},{}\n", generation, timestamp, neutron_energy,);

                    neutron_energies_file
                        .write(write_string.as_bytes())
                        .expect("Writing neutron energies to file.");
                }
            }

            if self.track_bins {
                write_bin_results_grid(
                    &self.bin_parameters,
                    &self.neutron_position_bins,
                    &format!("{}/bin_results_grid.csv", dir_path),
                );
            }
        }

        self.write_simulation_report(&dir_path, simulation_duration, halt_time);
    }
}
