use crate::diagnostics::geometry_diagnostics::GeometryDiagnostics;
use crate::diagnostics::halt_causes::SimulationHaltCauses;
use crate::utils::vectors::Vec3D;

use crate::diagnostics::{BinData, NeutronDiagnostics};

impl BinData {
    // Function to add two BinData instances
    pub fn add(&mut self, other: &BinData) {
        self.neutron_count += other.neutron_count;
        self.fission_count += other.fission_count;
    }
}

impl NeutronDiagnostics {
    pub fn new(
        estimate_k: bool,
        track_bins: bool,
        track_fission_positions: bool,
        track_from_generation: i64,
        bin_parameters: GeometryDiagnostics,
        initial_neutron_count: i64,
    ) -> NeutronDiagnostics {
        let bin_data = BinData::default();

        let neutron_position_bins = vec![
            bin_data;
            (bin_parameters.length_count + 1)
                * (bin_parameters.depth_count + 1)
                * (bin_parameters.height_count + 1)
        ];

        let max_generation_value = 0;
        let averaged_k = 0.0;
        let halt_cause = SimulationHaltCauses::default();
        let total_neutrons_tracked = 0;
        let total_fissions = 0;
        let average_power: f64 = 0.0;

        NeutronDiagnostics {
            neutron_generation_history: Vec::<i64>::new(),
            bin_parameters,
            neutron_position_bins,
            estimate_k,
            track_bins,
            max_generation_value,
            averaged_k,
            halt_cause,
            initial_neutron_count,
            total_neutrons_tracked,
            total_fissions,
            track_from_generation,
            power_generated: average_power,
            neutron_fission_locations: Vec::<Vec3D>::new(),
            track_fission_positions,
        }
    }

    pub fn get_total_fissions(&self) -> i64 {
        self.neutron_fission_locations.len() as i64
    }

    pub fn get_current_bin(&self, neutron_position: Vec3D) -> Option<usize> {
        self.bin_parameters.get_current_bin(neutron_position)
    }

    pub fn track_neutron_location_fission(
        &mut self,
        generation_number: i64,
        neutron_position: Vec3D,
    ) {
        if self.track_fission_positions && generation_number >= self.track_from_generation {
            self.neutron_fission_locations.push(neutron_position);
        }
    }

    pub fn track_neutron_bin_presence(&mut self, generation_number: i64, neutron_position: Vec3D) {
        if self.track_bins && generation_number >= self.track_from_generation {
            if let Some(current_bin) = self.get_current_bin(neutron_position) {
                self.neutron_position_bins[current_bin].neutron_count += 1
            }
        }
    }

    pub fn track_neutron_bin_fission(&mut self, generation_number: i64, neutron_position: Vec3D) {
        if self.track_bins && generation_number >= self.track_from_generation {
            if let Some(current_bin) = self.get_current_bin(neutron_position) {
                self.neutron_position_bins[current_bin].fission_count += 1
            }
        }
    }

    pub fn track_simulation_halt(
        &mut self,
        neutron_generation: i64,
        neutron_generation_history: Vec<i64>,
        halt_cause: SimulationHaltCauses,
    ) {
        self.neutron_generation_history = neutron_generation_history;
        self.total_neutrons_tracked = self.neutron_generation_history.iter().sum();
        self.halt_cause = halt_cause;
        self.max_generation_value = neutron_generation;
    }
}
