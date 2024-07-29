use crate::diagnostics::geometry_diagnostics::GeometryDiagnostics;
use crate::diagnostics::BinData;
use crate::utils::vectors::Vec3D;
use csv::Writer;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

/// Writing the bin results as a vector, the same as in memory.
/// Used for internal storage: can be directly deserialized and loaded in again.
pub fn write_bin_results_vector(neutron_position_bins: &Vec<BinData>, file_path: &Path) {
    let neutron_position_bin_file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(file_path)
        .expect("Opening neutron bins file.");

    let mut wtr = Writer::from_writer(neutron_position_bin_file);

    for bin_data in neutron_position_bins {
        wtr.serialize(bin_data)
            .expect("Failed to write bin data to CSV.");
    }

    wtr.flush().expect("Failed to flush writer.");
}

/// Write the bin results to a structured grid, with x,y,z pairs.
/// Difficult to re-serialize but can be loaded into ParaView VTKs.
pub fn write_bin_results_grid(
    geometry: &GeometryDiagnostics,
    simulation_bins: &Vec<BinData>,
    file_path: &Path,
) {
    let mut neutron_position_bin_file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(file_path)
        .expect("Opening neutron bins file.");

    neutron_position_bin_file
        .write("x,y,z,neutron_count,fission_count\n".as_bytes())
        .expect("Writing neutron bins headers.");

    for x_bin in 0..geometry.length_count {
        for y_bin in 0..geometry.depth_count {
            for z_bin in 0..geometry.height_count {
                let current_bin = x_bin
                    + y_bin * geometry.length_count
                    + z_bin * geometry.length_count * geometry.depth_count;

                let bin_data = &simulation_bins[current_bin];

                let x = geometry.x_min
                    + (x_bin as f64 / geometry.length_count as f64) * geometry.total_length;
                let y = geometry.y_min
                    + (y_bin as f64 / geometry.depth_count as f64) * geometry.total_depth;
                let z = geometry.z_min
                    + (z_bin as f64 / geometry.height_count as f64) * geometry.total_height;

                let write_string = format!(
                    "{:.5},{:.5},{:.5},{},{}\n",
                    x, y, z, bin_data.neutron_count, bin_data.fission_count
                );

                neutron_position_bin_file
                    .write(write_string.as_bytes())
                    .expect("Writing neutron position to file");
            }
        }
    }
}

/// Write the fission events to a file.
pub fn write_fission_vector(combined_fission_vector: Vec<Vec3D>, file_path: &Path) {
    let neutron_fissions_file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(file_path)
        .expect("Opening neutron fissions file.");

    let mut wtr = csv::Writer::from_writer(neutron_fissions_file);

    for fission_event in combined_fission_vector {
        wtr.serialize(fission_event)
            .expect("Writing neutron fission position to file");
    }

    wtr.flush().expect("Flushing CSV writer");
}
