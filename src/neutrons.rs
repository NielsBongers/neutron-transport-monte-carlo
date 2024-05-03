use crate::utils::vectors::Vec3D;

pub mod neutron_dynamics;
pub mod neutron_scheduler;
pub mod watt_distribution;

/// Implements all the information required to track the neutrons over time, and has a series of functions that allow for initialization, interaction with materials, and some utility functions.
#[derive(Default, Clone)]
pub struct Neutron {
    pub energy: f64,
    pub velocity: f64,
    pub time_step: f64,
    pub creation_time: f64,
    pub current_time: f64,

    pub distance_step: f64,

    pub position: Vec3D,
    pub direction: Vec3D,

    pub generation_number: i64,

    pub has_scattered: bool,
}
