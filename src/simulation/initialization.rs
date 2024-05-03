use crate::neutrons::Neutron;
use crate::simulation::Simulation;
use log::warn;

impl Simulation {
    /// Sampling an initial population from the initial neutron's position. This may encounter non-fissionable materials.
    /// In that case, the loop skips and continues until the specific number of neutrons is achieved.
    pub fn populate_initial_neutrons(&mut self, parent_neutron: &Neutron) {
        let mut trial_attempts = 0;
        let mut has_warned = false;

        while self.neutron_scheduler.total_neutron_count()
            < self.simulation_parameters.initial_neutron_count
        {
            trial_attempts += 1;
            if trial_attempts > self.simulation_parameters.initial_neutron_count * 1000
                && !has_warned
            {
                warn!("Less than 0.1% success rate at generating initial neutrons. Verify the initial position is valid. Code will continue to run, but may never finish.");
                has_warned = true;
            }

            // Getting the material.
            let (material_properties, _) = self
                .components
                .get_material_properties(&mut self.rng, &parent_neutron.position);

            if !material_properties.fissionable {
                continue;
            }

            let mut neutron = Neutron::default();
            neutron.initialize(
                &parent_neutron,
                material_properties.watt_a,
                material_properties.watt_b,
                &mut self.rng,
            );

            self.neutron_scheduler.add_neutron(neutron);
        }
    }

    /// Prepares for the simulation to be ran by adding neutrons, initializing the cache etc.
    pub fn prepare_simulation(&mut self) {
        // Setting up the first neutron.
        let mut parent_neutron = Neutron::default();
        parent_neutron.position = self.simulation_parameters.neutron_initial_position;
        parent_neutron.energy = 1e6;

        self.components
            .update_cache_properties(parent_neutron.energy);
        self.components.get_maximum_radius_squared();

        self.populate_initial_neutrons(&parent_neutron);

        self.neutron_scheduler.maximum_neutrons_per_generation =
            self.simulation_parameters.neutron_count_cap;
        self.neutron_scheduler.enforce_maximum_neutron_count =
            self.simulation_parameters.enforce_maximum_neutron_count;
    }
}
