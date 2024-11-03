use crate::diagnostics::NeutronDiagnostics;
use chrono::{DateTime, Local};
use log::{debug, error, info};
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use std::time::Duration;

impl NeutronDiagnostics {

    pub fn write_simulation_report(
        &mut self,
        dir_path: &Path,
        simulation_duration: Duration,
        halt_time: Option<f64>,
    ) {
        let mut simulation_report = OpenOptions::new()
            .create(true)
            .append(true)
            .open(format!("{}/simulation_report.dat", dir_path.display()))
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

        let total_fissions = self.get_total_fissions();

        self.calculate_power_production(halt_time);

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
            total_fissions,
            "Total energy produced:",
            self.total_energy,
            "Power:",
            self.power_generated,
            "Halt cause: ",
            self.halt_cause,
            "Estimate k:",
            self.estimate_k,
            "Track bins:",
            self.track_bins,
            "Track fission positions:",
            self.track_fission_positions,
        );

        simulation_report
            .write(report_line.as_bytes())
            .expect("Failed to write report to file.");

        info!("\n{}", report_line);
    }

    pub fn write_data(&mut self, simulation_duration: Duration, halt_time: Option<f64>) {
        let local_date_time: DateTime<Local> = Local::now();
        let date_time_string = local_date_time.format("%Y-%m-%d_%H-%M-%S.%f").to_string();
        let dir_path_string = format!("results/diagnostics/individual_runs/{}", date_time_string);
        let dir_path = Path::new(&dir_path_string);

        match fs::create_dir_all(&dir_path) {
            Err(why) => error!("! {:?}", why.kind()),
            Ok(_) => debug!("Successfully created directory {}", dir_path.display()),
        }

        if self.estimate_k {
            self.estimate_k();
        }

        self.write_simulation_report(&dir_path, simulation_duration, halt_time);
    }
}
