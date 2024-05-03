use crate::diagnostics::NeutronDiagnostics;
use log::warn;

impl NeutronDiagnostics {
    /// Estimates the multiplication coefficient _k_ based on data collected in ```generation_number``` during a simulation run.
    pub fn estimate_k(&mut self) -> Option<(f64, Vec<i64>)> {
        let max_generation_value: &i64 = self.generation_number.iter().max().unwrap_or(&0);

        if max_generation_value < &4 {
            warn!(
                "Too few datapoints: {}. Need at least 3 generations.",
                max_generation_value
            );
            return None;
        }

        let mut generation_count_vector: Vec<i64> = vec![0; *max_generation_value as usize];
        for neutron_generation_data in &self.generation_number {
            generation_count_vector[(*neutron_generation_data - 1) as usize] += 1;
        }

        self.generation_counts = generation_count_vector.clone();
        self.total_neutrons_tracked = self.generation_counts.iter().sum();

        let mut total_k_sum: f64 = 0.0;
        let mut k_sample_count: i64 = 0;

        for generation_index in
            (self.track_from_generation as usize + 1)..generation_count_vector.len()
        {
            let k_estimate: f64 = generation_count_vector[generation_index] as f64
                / generation_count_vector[generation_index - 1] as f64;

            total_k_sum += k_estimate;
            k_sample_count += 1;
        }

        let averaged_k: f64 = total_k_sum / k_sample_count as f64;

        // debug!("Averaged k = {:.2}", averaged_k);
        self.averaged_k = averaged_k;

        Some((averaged_k, generation_count_vector))
    }
}
