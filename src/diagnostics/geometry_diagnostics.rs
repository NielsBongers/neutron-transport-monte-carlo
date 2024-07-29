use crate::utils::config_loading::GridBinParametersTOML;
use crate::utils::vectors::Vec3D;

#[derive(Default, Clone)]
pub struct GeometryDiagnostics {
    pub x_min: f64,
    pub x_max: f64,
    pub y_min: f64,
    pub y_max: f64,
    pub z_min: f64,
    pub z_max: f64,

    pub length_count: usize,
    pub depth_count: usize,
    pub height_count: usize,

    pub total_length: f64,
    pub total_depth: f64,
    pub total_height: f64,

    pub delta_x: f64,
    pub delta_y: f64,
    pub delta_z: f64,
}

impl GeometryDiagnostics {
    pub fn new(grid_bin_parameters: GridBinParametersTOML) -> GeometryDiagnostics {
        let (x_min, x_max) = (
            grid_bin_parameters.center.x - grid_bin_parameters.total_length / 2.0,
            grid_bin_parameters.center.x + grid_bin_parameters.total_length / 2.0,
        );
        let (y_min, y_max) = (
            grid_bin_parameters.center.y - grid_bin_parameters.total_depth / 2.0,
            grid_bin_parameters.center.y + grid_bin_parameters.total_depth / 2.0,
        );
        let (z_min, z_max) = (
            grid_bin_parameters.center.z - grid_bin_parameters.total_height / 2.0,
            grid_bin_parameters.center.z + grid_bin_parameters.total_height / 2.0,
        );

        let length_count = grid_bin_parameters.length_count;
        let depth_count = grid_bin_parameters.depth_count;
        let height_count = grid_bin_parameters.height_count;

        let total_length = grid_bin_parameters.total_length;
        let total_depth = grid_bin_parameters.total_depth;
        let total_height = grid_bin_parameters.total_height;

        let delta_x = grid_bin_parameters.total_length / grid_bin_parameters.length_count as f64;
        let delta_y = grid_bin_parameters.total_depth / grid_bin_parameters.depth_count as f64;
        let delta_z = grid_bin_parameters.total_height / grid_bin_parameters.height_count as f64;

        GeometryDiagnostics {
            x_min,
            x_max,
            y_min,
            y_max,
            z_min,
            z_max,
            length_count,
            depth_count,
            height_count,
            total_length,
            total_depth,
            total_height,
            delta_x,
            delta_y,
            delta_z,
        }
    }

    pub fn get_current_bin(&self, neutron_position: Vec3D) -> Option<usize> {
        if !(neutron_position.x < self.x_min
            || neutron_position.x > self.x_max
            || neutron_position.y < self.y_min
            || neutron_position.y > self.y_max
            || neutron_position.z < self.z_min
            || neutron_position.z > self.z_max)
        {
            let x_bin = ((neutron_position.x - self.x_min) / (self.x_max - self.x_min)
                * self.length_count as f64) as usize;
            let y_bin = ((neutron_position.y - self.y_min) / (self.y_max - self.y_min)
                * self.depth_count as f64) as usize;
            let z_bin = ((neutron_position.z - self.z_min) / (self.z_max - self.z_min)
                * self.height_count as f64) as usize;

            let current_bin =
                x_bin + y_bin * self.length_count + z_bin * self.length_count * self.depth_count;

            return Some(current_bin);
        }
        None
    }

    pub fn index_to_coordinates(
        &self,
        x_bin: usize,
        y_bin: usize,
        z_bin: usize,
    ) -> (f64, f64, f64) {
        let x = self.x_min + (x_bin as f64 / self.length_count as f64) * self.total_length;
        let y = self.y_min + (y_bin as f64 / self.depth_count as f64) * self.total_depth;
        let z = self.z_min + (z_bin as f64 / self.height_count as f64) * self.total_height;

        (x, y, z)
    }

    pub fn bins_to_index(&self, x_bin: &usize, y_bin: &usize, z_bin: &usize) -> usize {
        x_bin + y_bin * self.length_count + z_bin * self.length_count * self.depth_count
    }
}
