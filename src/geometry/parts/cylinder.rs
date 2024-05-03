use serde::Deserialize;
use serde::Serialize;

use crate::geometry::components::BoundingBox;
use crate::geometry::components::PartComposition;
use crate::materials::material_properties::MaterialNames;
use crate::utils::vectors::Vec3D;

#[derive(Debug, Serialize, Deserialize)]
pub struct Cylinder {
    pub center: Vec3D,
    pub direction: Vec3D,
    pub length: f64,
    pub radius: f64,
    pub bounding_box: BoundingBox,
    pub name: String,
    pub material_name: MaterialNames,
    pub material_composition_vector: Vec<PartComposition>,
    pub order: i32,
    squared_radius: f64,
    half_length: f64,
}

impl Cylinder {
    pub fn new(
        center: Vec3D,
        direction: Vec3D,
        length: f64,
        radius: f64,
        material_name: MaterialNames,
        material_composition_vector: Vec<PartComposition>,
        order: i32,
    ) -> Self {
        let squared_radius = f64::powi(radius, 2);
        let half_length: f64 = length / 2.0;

        let end1 = center.add(direction.scalar_dot(half_length));
        let end2 = center.add(direction.scalar_dot(-half_length));

        let min = end1.min(end2).scalar_add(-radius);
        let max = end1.max(end2).scalar_add(radius);

        let bounding_box = BoundingBox { min, max };

        // debug!("Min: {}\nMax: {}", min, max);

        let name: String = "Cylinder".to_string();

        Self {
            center,
            direction,
            length,
            radius,
            bounding_box,
            name,
            material_name,
            material_composition_vector,
            order,
            squared_radius,
            half_length,
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

    /// This function takes the neutron position and calculates if the neutron is currently inside the cylinder.
    /// This was a bit difficult to implement. We first calculate the relative position to the center of the cylinder.
    /// Then, we calculate the parallel component to the unit vector defined for the cylinder's angle, with a dot product.
    /// From that, we use Pythagoras to determine the perpendicular component, which amounts to the radius.
    /// To speed up computation, we pre-compute the squared radius and halved length.
    pub fn is_inside(&self, neutron_position: &Vec3D) -> bool {
        if !self.is_inside_bounding_box(neutron_position) {
            // debug!("Outside bounding box!");
            return false;
        }

        let relative_position = neutron_position.subtract(self.center);
        let parallel_component = relative_position.dot(self.direction);
        let perpendicular_component_squared =
            relative_position.norm_squared() - parallel_component.powi(2);

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

        // debug!("Perpendicular: {}", perpendicular_component_squared);

        let is_inside_radius: bool = perpendicular_component_squared <= self.squared_radius;
        let is_inside_length: bool = parallel_component.abs() <= self.half_length;

        is_inside_radius && is_inside_length
    }
}
