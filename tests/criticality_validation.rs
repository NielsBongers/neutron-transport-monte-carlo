use log::info;
use nuclear;
use nuclear::diagnostics::geometry_diagnostics::GeometryDiagnostics;
use nuclear::diagnostics::NeutronDiagnostics;
use nuclear::geometry::components::Components;
use nuclear::geometry::presets::create_spheres::{create_default_sphere, create_reference_sphere};
use nuclear::materials::material_properties::get_material_data_vector;
use nuclear::neutrons::neutron_scheduler::NeutronScheduler;
use nuclear::simulation::Simulation;
use nuclear::utils::config_loading::load_config;
use rand::rngs::SmallRng;
use rand::SeedableRng;
use std::path::Path;

#[test]
fn godiva_test() {
    let rng = SmallRng::from_entropy();

    // Loading config
    let config = load_config(Path::new("config/simulation/reference.toml"));

    let simulation_parameters = config.simulation_parameters;
    let neutron_bin_parameters = config.neutron_bins;

    // Required structs.
    let components: Components =
        Components::new(get_material_data_vector(), create_reference_sphere());
    components.check_material_fractions_sum();

    let neutron_scheduler: NeutronScheduler = NeutronScheduler::default();
    let bin_parameters = GeometryDiagnostics::new(neutron_bin_parameters);

    let neutron_diagnostics: NeutronDiagnostics = NeutronDiagnostics::new(
        simulation_parameters.estimate_k,
        simulation_parameters.track_bins,
        simulation_parameters.track_fission_positions,
        simulation_parameters.track_from_generation,
        bin_parameters,
        simulation_parameters.initial_neutron_count,
    );

    // Instantiating simulation.
    let mut simulation: Simulation = Simulation {
        rng,
        components,
        neutron_scheduler,
        neutron_diagnostics,
        simulation_parameters,
    };

    use std::time::Instant;
    let now = Instant::now();
    let simulation_result: bool = simulation.run_simulation();
    info!("Simulation finished: {}", simulation_result);
    let simulation_time = now.elapsed();
    info!("Simulation time: {:.2?}", simulation_time);

    assert!(simulation_result);

    let (k_estimate, _) = simulation
        .neutron_diagnostics
        .estimate_k()
        .expect("Too few generations to give a reasonable k-estimate.");
    let k_known = 1.0099;
    let error_margin: f64 = 1.05;

    assert!(k_estimate / k_known <= error_margin);
}

#[test]
fn infinite_medium_test() {
    let rng = SmallRng::from_entropy();

    // Loading config
    let config = load_config(Path::new("config/simulation/reference.toml"));

    let simulation_parameters = config.simulation_parameters;
    let neutron_bin_parameters = config.neutron_bins;

    // Required structs.
    let components: Components =
        Components::new(get_material_data_vector(), create_default_sphere(1000.));
    components.check_material_fractions_sum();

    let neutron_scheduler: NeutronScheduler = NeutronScheduler::default();
    let bin_parameters = GeometryDiagnostics::new(neutron_bin_parameters);

    let neutron_diagnostics: NeutronDiagnostics = NeutronDiagnostics::new(
        simulation_parameters.estimate_k,
        simulation_parameters.track_bins,
        simulation_parameters.track_fission_positions,
        simulation_parameters.track_from_generation,
        bin_parameters,
        simulation_parameters.initial_neutron_count,
    );

    // Instantiating simulation.
    let mut simulation: Simulation = Simulation {
        rng,
        components,
        neutron_scheduler,
        neutron_diagnostics,
        simulation_parameters,
    };

    use std::time::Instant;
    let now = Instant::now();
    let simulation_result: bool = simulation.run_simulation();
    info!("Simulation finished: {}", simulation_result);
    let simulation_time = now.elapsed();
    info!("Simulation time: {:.2?}", simulation_time);

    assert!(simulation_result);

    let (k_estimate, _) = simulation
        .neutron_diagnostics
        .estimate_k()
        .expect("Too few generations to give a reasonable k-estimate.");

    info!("k estimate: {}", k_estimate);

    let k_known = 2.5;
    let error_margin: f64 = 1.05;

    assert!(k_estimate / k_known <= error_margin);
}
