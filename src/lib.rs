//! # Neutron transport Monte Carlo
//!
//! Neutron transport Monte Carlo written in Rust, intended for nuclear reactor simulations.
//!

/// Handles diagnostics functions for tracking neutron behavior over time.
pub mod diagnostics;
/// Different part types with material properties through the use of simple constructive solid geometry tools.
pub mod geometry;
/// Heat diffusion simulation.
pub mod heat_diffusion;
/// Material data and energy-dependent properties for common materials used in nuclear engineering.
pub mod materials;
/// Neutron simulation with scattering, absorption and fission.
pub mod neutrons;
/// Overarching simulation module that integrates the other modules.
pub mod simulation;
/// Utilities for file-handling, vectors etc.
pub mod utils;
