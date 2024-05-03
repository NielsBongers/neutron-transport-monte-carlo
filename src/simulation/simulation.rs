use crate::neutrons::neutron_dynamics::InteractionTypes;
use crate::neutrons::Neutron;
use crate::simulation::Simulation;

impl Simulation {
    /// Runs the main simulation loop until certain termination conditions are met.
    pub fn run_simulation(&mut self) -> bool {
        self.prepare_simulation();

        let mut tracked_neutron_generation = 0;

        // Outer loop over the neutron vector until we either hit a specific limit or we run out of neutrons.
        loop {
            let mut iteration = 0;

            if self.neutron_scheduler.is_empty() {
                // debug!("No more neutrons.");
                self.neutron_diagnostics.track_simulation_halt(
                    0 as i64,
                    tracked_neutron_generation as i64,
                    self.simulation_parameters.neutron_generation_cap,
                    self.simulation_parameters.neutron_count_cap,
                );
                return false;
            }

            // Getting the earliest neutron.
            let neutron = self.neutron_scheduler.get_neutron(&mut self.rng);

            // General diagnostics.
            tracked_neutron_generation = neutron.generation_number;

            // Updating the material properties cache for the current neutron's energy.
            self.components.update_cache_properties(neutron.energy);

            // Inner loop over the current neutron.
            loop {
                iteration += 1;
                neutron.current_time = neutron.creation_time + iteration as f64 * neutron.time_step;

                neutron.translate();

                // Applying diagnostics.
                self.neutron_diagnostics.track_neutron_energies(
                    neutron.generation_number,
                    neutron.current_time,
                    neutron.energy,
                );
                self.neutron_diagnostics
                    .track_neutron_position(neutron.generation_number, neutron.position);
                self.neutron_diagnostics
                    .track_neutron_bin_presence(neutron.generation_number, neutron.position);

                // Updating the caches in case the neutron has encountered elastic scattering, changing its energy.
                if neutron.has_scattered {
                    self.components.update_cache_properties(neutron.energy);
                    neutron.has_scattered = false;
                }

                // Getting material properties.
                // This gives the total cross-section and the selected material, for if the material actually interacts.
                let (material_properties, composition_total_cross_section) = self
                    .components
                    .get_material_properties(&mut self.rng, &neutron.position);

                // Interacting with the material.
                let interaction_type: InteractionTypes = neutron.interact(
                    &material_properties,
                    composition_total_cross_section,
                    self.components.simulation_range_squared,
                    &mut self.rng,
                );

                // Responding to the interactions types.
                if interaction_type == InteractionTypes::None {
                    continue;
                }

                if interaction_type == InteractionTypes::Escaped {
                    self.neutron_scheduler.remove_neutron(0);
                    // debug!("Escaped");
                    break;
                }

                if interaction_type == InteractionTypes::Scattering {
                    neutron.scatter(
                        material_properties.atomic_mass,
                        &mut self.rng,
                        self.simulation_parameters.maximum_neutron_energy_difference,
                    );
                    // debug!("Scattering");
                }

                if interaction_type == InteractionTypes::Absorption {
                    self.neutron_scheduler.remove_neutron(0);
                    // debug!("Absorbed!");
                    break;
                }

                if interaction_type == InteractionTypes::Fission {
                    // debug!("Fissioning");
                    self.neutron_diagnostics
                        .track_neutron_bin_fission(neutron.generation_number, neutron.position);

                    let fission_count: i32 = neutron
                        .get_neutron_fission_count(material_properties.nu_bar, &mut self.rng);

                    let mut new_neutron: Neutron = Neutron::default();
                    new_neutron.initialize(
                        &neutron,
                        material_properties.watt_a,
                        material_properties.watt_b,
                        &mut self.rng,
                    );

                    self.neutron_scheduler.remove_neutron(0);

                    for _ in 0..fission_count {
                        self.neutron_diagnostics.track_creation(
                            new_neutron.creation_time,
                            new_neutron.generation_number,
                        );
                        self.neutron_scheduler.add_neutron(new_neutron.clone());
                    }
                    break;
                }

                if let Some(halt_time) = self.simulation_parameters.halt_time {
                    if neutron.current_time > halt_time {
                        // debug!("Halting neutron at {}", neutron.current_time);
                        self.neutron_scheduler.remove_neutron(0);
                        break;
                    }
                }
            }

            let total_neutron_count = self.neutron_scheduler.current_neutron_count();

            if total_neutron_count > self.simulation_parameters.neutron_count_cap
                && !self.simulation_parameters.enforce_maximum_neutron_count
            {
                self.neutron_diagnostics.track_simulation_halt(
                    total_neutron_count as i64,
                    tracked_neutron_generation as i64,
                    self.simulation_parameters.neutron_generation_cap,
                    self.simulation_parameters.neutron_count_cap,
                );
                return true;
            }

            if tracked_neutron_generation > self.simulation_parameters.neutron_generation_cap {
                self.neutron_diagnostics.track_simulation_halt(
                    total_neutron_count as i64,
                    tracked_neutron_generation as i64,
                    self.simulation_parameters.neutron_generation_cap,
                    self.simulation_parameters.neutron_count_cap,
                );
                return true;
            }
        }
    }
}
