use crate::geometry::components::BoundingBox;
use crate::geometry::components::PartComposition;
use crate::materials::material_properties::MaterialNames;
use crate::utils::vectors::Vec3D;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Sphere {
    pub center: Vec3D,
    pub radius: f64,
    pub bounding_box: BoundingBox,
    pub name: String,
    pub squared_radius: f64,
    pub material_name: MaterialNames,
    pub material_composition_vector: Vec<PartComposition>,
    pub order: i32,
}

impl Sphere {
    pub fn new(
        center: Vec3D,
        radius: f64,
        material_name: MaterialNames,
        material_composition_vector: Vec<PartComposition>,
        order: i32,
    ) -> Self {
        let squared_radius = f64::powi(radius, 2);

        let bottom_corner = Vec3D {
            x: -radius,
            y: -radius,
            z: -radius,
        };

        let top_corner = Vec3D {
            x: radius,
            y: radius,
            z: radius,
        };

        let min = bottom_corner.add(center);
        let max = top_corner.add(center);

        let bounding_box = BoundingBox { min, max };
        // debug!("Min: {}\nMax: {}", min, max);

        let name: String = "Sphere".to_string();

        Self {
            center,
            radius,
            bounding_box,
            name,
            squared_radius,
            material_name,
            material_composition_vector,
            order,
        }
    }

    pub fn is_inside_bounding_box(&self, neutron_position: &Vec3D) -> bool {
        if neutron_position.x < self.bounding_box.min.x
            || neutron_position.x > self.bounding_box.max.x
        {
            return false;
        }
        if neutron_position.y < self.bounding_box.min.y
            || neutron_position.y > self.bounding_box.max.y
        {
            return false;
        }
        if neutron_position.z < self.bounding_box.min.z
            || neutron_position.z > self.bounding_box.max.z
        {
            return false;
        }
        true
    }

    pub fn is_inside(&self, neutron_position: &Vec3D) -> bool {
        if !self.is_inside_bounding_box(neutron_position) {
            return false;
        }

        let relative_radius_squared = neutron_position.subtract(self.center).norm_squared();

        // debug!(
        //     "Relative position norm: {}",
        //     f64::powi(parallel_component, 2)
        // );
        // debug!(
        //     "Perpendicular component: {}",
        //     perpendicular_component_squared
        // );
        // debug!("Parallel component: {}", parallel_component);
        // debug!("Relative position: {}", relative_position);

        let is_inside_radius: bool = relative_radius_squared <= self.squared_radius;

        is_inside_radius
    }
}
