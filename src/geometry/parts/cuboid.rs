use serde::{Deserialize, Serialize};

use crate::geometry::components::BoundingBox;
use crate::geometry::components::PartComposition;
use crate::materials::material_properties::MaterialNames;
use crate::utils::vectors::Vec3D;

#[derive(Debug, Serialize, Deserialize)]
pub struct Cuboid {
    pub center: Vec3D,
    pub bounding_box: BoundingBox,
    pub name: String,
    pub material_name: MaterialNames,
    pub material_composition_vector: Vec<PartComposition>,
    pub order: i32,
}

impl Cuboid {
    pub fn new(
        center: Vec3D,
        width: f64,
        depth: f64,
        height: f64,
        material_name: MaterialNames,
        material_composition_vector: Vec<PartComposition>,
        order: i32,
    ) -> Self {
        let half_width = width / 2.0;
        let half_depth = depth / 2.0;
        let half_height = height / 2.0;

        let half_vector = Vec3D {
            x: half_width,
            y: half_depth,
            z: half_height,
        };

        let min = center.subtract(half_vector);
        let max = center.add(half_vector);

        let bounding_box = BoundingBox { min, max };
        // debug!("Min: {}\nMax: {}", min, max);

        let name: String = "Cuboid".to_string();

        Self {
            center,
            bounding_box,
            name,
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
        } else {
            return true;
        }
    }
}
