use crate::diagnostics::NeutronDiagnostics;
use log::warn;

impl NeutronDiagnostics {
    /// Estimates the multiplication coefficient _k_ based on data collected in the neutron scheduler during a simulation run.
    /// At the end of the simulation, this vector is passed from the scheduler to diagnostics via ```track_simulation_halt```.
    pub fn estimate_k(&mut self) -> Option<(f64, Vec<i64>)> {
        let maximum_generation = self.neutron_generation_counts.len();

        if (maximum_generation as i64) < self.track_from_generation {
            warn!("Only {} generations available while tracking starts at {}. k-estimation will be skipped.", maximum_generation, self.track_from_generation);
            return None;
        }

        if maximum_generation < 4 {
            warn!(
                "Only {} generations recorded - k-estimation may be inaccurate.",
                maximum_generation
            );
        }

        let mut k_estimate_vector = Vec::<f64>::new();

        for generation_count_window in
            self.neutron_generation_counts[(self.track_from_generation as usize)..].windows(2)
        {
            let previous_generation_count = generation_count_window[0];
            let current_generation_count = generation_count_window[1];

            let k_estimate = (current_generation_count as f64) / (previous_generation_count as f64);

            k_estimate_vector.push(k_estimate);
        }

        if k_estimate_vector.len() < 2 {
            warn!("No neutron generations logged - currently starting from generation {}. Is the neutron cap high enough? k estimate may not work.", self.track_from_generation);
            return None;
        }

        let averaged_k: f64 =
            k_estimate_vector.iter().sum::<f64>() / k_estimate_vector.len() as f64;

        self.averaged_k = averaged_k;

        Some((averaged_k, self.neutron_generation_counts.clone()))
    }

    pub fn calculate_power_production(&mut self, halt_time: Option<f64>) {
        let total_fissions = self.get_total_fissions();
        let energy_per_fission = 1.9341e+8; // eV
        let ev_to_joule = 1.60218e-19;

        self.total_energy = total_fissions as f64 * energy_per_fission * ev_to_joule; // J produced

        if let Some(halt_time) = halt_time {
            self.power_generated = self.total_energy / halt_time;
        }
    }

    pub fn post_process(&mut self, halt_time: Option<f64>) {
        if self.estimate_k {
            self.estimate_k();
        }

        if halt_time.is_some() {
            self.calculate_power_production(halt_time);
        }
    }
}
