use log::warn;
use rand::{distributions::Uniform, Rng};
use std::fs::OpenOptions;
use std::io::Write;

/// PDF for the Watt distribution.
fn watt_distribution(a: f64, b: f64, energy: f64) -> f64 {
    let mev_energy = energy / 1e6;

    let p = 2.0 * f64::exp(-a * b / 4.0) / (f64::sqrt(std::f64::consts::PI * f64::powi(a, 3) * b))
        * f64::exp(-mev_energy / a)
        * f64::sinh(f64::sqrt(b * mev_energy));

    p
}

/// Rejection sampling for the Watt distribution.
pub fn rejection_sample_watt(a: f64, b: f64, rng: &mut rand::rngs::SmallRng) -> Option<f64> {
    let maximum_energy = 1.5e7;
    let max_iterations = 1e3 as i64;

    let uniform: Uniform<f64> = Uniform::new(0.0, maximum_energy);

    for (_iteration_counter, _) in (0..max_iterations).enumerate() {
        let random_energy = rng.sample(uniform);
        let watt_probability = watt_distribution(a, b, random_energy);
        let rejection_criterion = rng.gen::<f64>();

        if rejection_criterion <= watt_probability {
            return Some(random_energy);
        }
    }

    warn!("Unable to find Watt distribution - rejection sampling failed within alotted iterations ({}) for (a: {}, b: {}). Returning None.", max_iterations, a, b);
    None
}

/// Sampling a specified number of energies according to the Watt distribution for a given _a_ and _b_ and saving them to a file.
fn _create_watt_samples(
    _count: i64,
    a_value: f64,
    b_value: f64,
    _energy: f64,
    rng: &mut rand::rngs::SmallRng,
) -> () {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("watt_distribution.csv")
        .expect("Unable to open Watt distribution file");

    for _ in 0..1e6 as i64 {
        let watt_probability = rejection_sample_watt(a_value, b_value, rng);

        match watt_probability {
            Some(probability) => {
                writeln!(file, "{}", probability / 1e6).unwrap();
            }
            None => continue,
        }
    }
}
