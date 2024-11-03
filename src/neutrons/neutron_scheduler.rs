use crate::neutrons::Neutron;
use log::debug;
use rand::seq::SliceRandom;
/// Stores the neutrons and schedules their runs.
/// This consists of two queues, which are swapped whenever a generation runs out.
/// If the option to enforce a maximum number of neutrons per generation is enabled, the filled queue is first shuffled, then truncated.
/// The swapping is done with the ```queue_selector``` boolean.
#[derive(Default)]
pub struct NeutronScheduler {
    pub neutron_queue_a: Vec<Neutron>,
    pub neutron_queue_b: Vec<Neutron>,

    pub neutron_generation_history: Vec<i64>,

    pub queue_selector: bool,

    pub specified_neutron_count: i64,

    pub variance_reduction: bool,
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

    fn shuffle_and_truncate(&mut self, rng: &mut rand::rngs::SmallRng) {
        // debug!(
        //     "Shuffling and truncating - we were at {} neutrons before",
        //     self.current_neutron_count()
        // );

        let target_queue = match self.queue_selector {
            false => &mut self.neutron_queue_a,
            true => &mut self.neutron_queue_b,
        };

        target_queue.shuffle(rng);
        target_queue.truncate(self.specified_neutron_count as usize)
    }

    fn shuffle_and_duplicate(&mut self, rng: &mut rand::rngs::SmallRng) {
        // debug!("Shuffling and duplicating");

        let target_queue = match self.queue_selector {
            false => &mut self.neutron_queue_a,
            true => &mut self.neutron_queue_b,
        };

        // Only called if target_queue.len() < specified_neutron_count, so this will never be negative.
        let mut neutrons_to_add = self.specified_neutron_count as usize - target_queue.len();

        while neutrons_to_add > 0 {
            // Shuffling each time: probably not necessary.
            target_queue.shuffle(rng);
            let current_queue_length = target_queue.len();
            let chunk_size = current_queue_length.min(neutrons_to_add);

            target_queue.extend_from_within(0..chunk_size);

            neutrons_to_add -= chunk_size;
        }
    }

    fn track_neutron_population_history(&mut self) {
        let current_neutron_count = self.total_neutron_count();
        self.neutron_generation_history.push(current_neutron_count);
    }

    fn check_queue_flip(&mut self, rng: &mut rand::rngs::SmallRng) {
        let neutron_count_current_queue = match self.queue_selector {
            false => self.neutron_queue_a.len(),
            true => self.neutron_queue_b.len(),
        };

        if neutron_count_current_queue == 0 {
            self.queue_selector = !self.queue_selector;

            self.track_neutron_population_history();

            debug!(
                "Currently at generation {}",
                self.neutron_generation_history.len()
            );

            if self.variance_reduction {
                if self.current_neutron_count() > self.specified_neutron_count {
                    // If we have too many neutrons, reduce to the specified count.
                    self.shuffle_and_truncate(rng);
                }

                if self.current_neutron_count() < self.specified_neutron_count {
                    // If we have too few, sample from the distribution and increase.
                    self.shuffle_and_duplicate(rng);
                }
            }
        }

        // debug!("Flipping queue: currently at {}", self.queue_selector);
    }

    // Gets a neutron from a specified neutron generation.
    pub fn get_neutron(&mut self, rng: &mut rand::rngs::SmallRng) -> &mut Neutron {
        self.check_queue_flip(rng);

        let neutron = match self.queue_selector {
            false => &mut self.neutron_queue_a[0],
            true => &mut self.neutron_queue_b[0],
        };

        neutron
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
