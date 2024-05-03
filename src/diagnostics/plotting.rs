use crate::diagnostics::geometry_diagnostics::GeometryDiagnostics;
use crate::materials::material_properties::map_indices_to_enum;
use crate::simulation::Simulation;
use crate::utils::vectors::Vec3D;
use log::{debug, error, info};
use std::fs;
use std::fs::create_dir_all;
use std::fs::OpenOptions;
use std::io::Write;

pub fn plot_geometry(simulation: &mut Simulation, plot_parameters: GeometryDiagnostics) {
    debug!("Starting geometry plotting.");

    simulation.components.update_cache_properties(1e6);
    simulation.components.get_maximum_radius_squared();

    let x_step =
        (plot_parameters.x_max - plot_parameters.x_min) / plot_parameters.length_count as f64;
    let y_step =
        (plot_parameters.y_max - plot_parameters.y_min) / plot_parameters.depth_count as f64;
    let z_step =
        (plot_parameters.z_max - plot_parameters.z_min) / plot_parameters.height_count as f64;

    let path = "results/geometry/";
    create_dir_all(path).expect("Failed to create geometry directory.");

    let mut geometry_file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(format!("results/geometry/geometry.csv"))
        .expect("Failed to open geometry data file.");

    geometry_file
        .write("x,y,z,index\n".as_bytes())
        .expect("Writing geometry material data headers.");

    for x_bin in 0..plot_parameters.length_count {
        for y_bin in 0..plot_parameters.depth_count {
            for z_bin in 0..plot_parameters.height_count {
                let x = plot_parameters.x_min + x_step * x_bin as f64;
                let y = plot_parameters.y_min + y_step * y_bin as f64;
                let z = plot_parameters.z_min + z_step * z_bin as f64;

                let neutron_position = Vec3D { x, y, z };

                let (material_index, _) = simulation
                    .components
                    .get_material_index(&mut simulation.rng, &neutron_position);

                let write_string = format!("{:.5},{:.5},{:.5},{}\n", x, y, z, material_index);

                geometry_file
                    .write(write_string.as_bytes())
                    .expect("Failed to write geometry to file.");
            }
        }
    }
    info!("Completed geometry plotting.");
}

/// Largely superseded by ```plot_geometry``` but in case ParaView is not available, this still provides an alternative to plot slices natively, which can subsequently be plotted by Matplotlib.
pub fn plot_geometry_slice(simulation: &mut Simulation) {
    simulation.components.update_cache_properties(1e6);
    simulation.components.get_maximum_radius_squared();

    let center = Vec3D::default();
    let span_1 = Vec3D {
        x: 1.0,
        y: 0.0,
        z: 0.0,
    };

    let span_2 = Vec3D {
        x: 0.0,
        y: 1.0,
        z: 0.0,
    };

    let span_dot_product = span_1.dot(span_2);
    assert_eq!(span_dot_product, 0.0);

    let min_parameter = -1.0;
    let max_parameter = 1.0;
    let step_size = 0.01;

    let step_count: i64 = ((max_parameter - min_parameter) / step_size) as i64 + 1;

    match fs::create_dir_all("results/geometry") {
        Err(why) => error!("! {:?}", why.kind()),
        Ok(_) => debug!("Successfully created directory."),
    }

    let mut geometry_file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(format!("results/geometry/plane.csv"))
        .expect("Opening diagnostics file.");

    geometry_file
        .write("x,y,z,material_index\n".as_bytes())
        .expect("Writing geometry header.");

    for i in 0..step_count {
        for j in 0..step_count {
            let scalar_1 = min_parameter + i as f64 * step_size;
            let scalar_2 = min_parameter + j as f64 * step_size;

            let span_1_position: Vec3D = span_1.scalar_dot(scalar_1);
            let span_2_position: Vec3D = span_2.scalar_dot(scalar_2);

            let combined_position: Vec3D = center.add(span_1_position.add(span_2_position));

            let (material_index, _) = simulation
                .components
                .get_material_index(&mut simulation.rng, &combined_position);

            if material_index != 0 {
                debug!(
                    "Currently at x, y: {:.2}, {:.2}. Encountering: {:?}",
                    combined_position.x,
                    combined_position.y,
                    map_indices_to_enum(material_index)
                );
            }

            geometry_file
                .write(
                    format!(
                        "{},{},{},{}\n",
                        combined_position.x,
                        combined_position.y,
                        combined_position.z,
                        material_index,
                    )
                    .as_bytes(),
                )
                .expect("Writing slice data to file.");
        }
    }
}
