use rand::seq::SliceRandom;

use crate::neutrons::Neutron;

/// Stores the neutrons and schedules their runs.
/// This consists of two queues, which are swapped whenever a generation runs out.
/// If the option to enforce a maximum number of neutrons per generation is enabled, the filled queue is first shuffled, then truncated.
/// The swapping is done with the ```queue_selector``` boolean.
#[derive(Default)]
pub struct NeutronScheduler {
    pub neutron_queue_a: Vec<Neutron>,
    pub neutron_queue_b: Vec<Neutron>,

    pub queue_selector: bool,

    pub maximum_neutrons_per_generation: i64,
    pub enforce_maximum_neutron_count: bool,
}

impl NeutronScheduler {
    /// Adds a neutron to the _next_ generation, taking the other queue by inverting the queue selector boolean.
    pub fn add_neutron(&mut self, neutron: Neutron) {
        // match !self.queue_selector {
        //     false => debug!("Adding to queue A"),
        //     true => debug!("Adding to queue B"),
        // }

        match !self.queue_selector {
            false => self.neutron_queue_a.push(neutron),
            true => self.neutron_queue_b.push(neutron),
        };
    }

    pub fn remove_neutron(&mut self, neutron_index: usize) {
        match self.queue_selector {
            false => self.neutron_queue_a.swap_remove(neutron_index),
            true => self.neutron_queue_b.swap_remove(neutron_index),
        };
    }

    pub fn shuffle_and_truncate(&mut self, rng: &mut rand::rngs::SmallRng) {
        // debug!(
        //     "Shuffling and truncating - we were at {} neutrons before",
        //     self.current_neutron_count()
        // );

        match self.queue_selector {
            false => {
                self.neutron_queue_a.shuffle(rng);
                self.neutron_queue_a
                    .truncate(self.maximum_neutrons_per_generation as usize)
            }
            true => {
                self.neutron_queue_b.shuffle(rng);
                self.neutron_queue_b
                    .truncate(self.maximum_neutrons_per_generation as usize)
            }
        }
    }

    pub fn check_queue_flip(&mut self, rng: &mut rand::rngs::SmallRng) {
        let neutron_count_current_queue = match self.queue_selector {
            false => self.neutron_queue_a.len(),
            true => self.neutron_queue_b.len(),
        };

        if neutron_count_current_queue == 0 {
            self.queue_selector = !self.queue_selector;

            if self.current_neutron_count() > self.maximum_neutrons_per_generation
                && self.enforce_maximum_neutron_count
            {
                self.shuffle_and_truncate(rng);
            }
        }

        // debug!("Flipping queue: currently at {}", self.queue_selector);
    }

    // Gets a neutron from a specified neutron generation.
    pub fn get_neutron(&mut self, rng: &mut rand::rngs::SmallRng) -> &mut Neutron {
        self.check_queue_flip(rng);

        match self.queue_selector {
            false => &mut self.neutron_queue_a[0],
            true => &mut self.neutron_queue_b[0],
        }
    }

    pub fn is_empty(&self) -> bool {
        self.neutron_queue_a.is_empty() && self.neutron_queue_b.is_empty()
    }

    pub fn current_neutron_count(&self) -> i64 {
        // debug!(
        //     "Queue A: {}. Queue B: {}",
        //     self.neutron_queue_a.len(),
        //     self.neutron_queue_b.len()
        // );

        match self.queue_selector {
            false => self.neutron_queue_a.len() as i64,
            true => self.neutron_queue_b.len() as i64,
        }
    }

    pub fn total_neutron_count(&self) -> i64 {
        (self.neutron_queue_a.len() + self.neutron_queue_b.len()) as i64
    }
}
