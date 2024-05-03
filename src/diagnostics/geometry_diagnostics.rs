use crate::utils::vectors::Vec3D;

#[derive(Default)]
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
}

impl GeometryDiagnostics {
    pub fn new(
        length_count: usize,
        depth_count: usize,
        height_count: usize,
        center: Vec3D,
        total_length: f64,
        total_depth: f64,
        total_height: f64,
    ) -> GeometryDiagnostics {
        let (x_min, x_max) = (center.x - total_length / 2.0, center.x + total_length / 2.0);
        let (y_min, y_max) = (center.y - total_depth / 2.0, center.y + total_depth / 2.0);
        let (z_min, z_max) = (center.z - total_height / 2.0, center.z + total_height / 2.0);

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
        }
    }
}
