use crate::diagnostics::NeutronDiagnostics;
use log::warn;

impl NeutronDiagnostics {
    /// Estimates the multiplication coefficient _k_ based on data collected in the neutron scheduler during a simulation run.
    /// At the end of the simulation, this vector is passed from the scheduler to diagnostics via ```track_simulation_halt```.
    pub fn estimate_k(&mut self) -> Option<(f64, Vec<i64>)> {
        let maximum_generation = self.neutron_generation_history.len();

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
            self.neutron_generation_history[(self.track_from_generation as usize)..].windows(2)
        {
            let previous_generation_count = generation_count_window[0];
            let current_generation_count = generation_count_window[1];

            let k_estimate = (current_generation_count as f64) / (previous_generation_count as f64);

            k_estimate_vector.push(k_estimate);
        }

        let averaged_k: f64 =
            k_estimate_vector.iter().sum::<f64>() / k_estimate_vector.len() as f64;

        self.averaged_k = averaged_k;

        Some((averaged_k, self.neutron_generation_history.clone()))
    }
}
