use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::geometry::parts::parts::PartTypes;
use crate::materials::material_data::MaterialData;
use crate::materials::material_properties::{
    map_enum_to_indices, MaterialNames, MaterialProperties,
};
use crate::utils::vectors::Vec3D;

use log::debug;

/// Basic bounding-box for faster rejection: if the neutron is outside the bounding box, the more complex check is skipped.
#[derive(Debug, Serialize, Deserialize)]
pub struct BoundingBox {
    pub min: Vec3D,
    pub max: Vec3D,
}

/// Part composition for mixed materials.
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct PartComposition {
    pub material_name: MaterialNames,
    pub material_fraction: f64,
}

/// Struct that contains the material data, cached properties and parts - essentially all the geometry.
#[derive(Debug)]
pub struct Components {
    pub material_data_vector: Vec<MaterialData>,
    pub cached_material_properties: Vec<MaterialProperties>,
    pub parts_vector: Vec<PartTypes>,
    pub cache_initialized: bool,
    pub simulation_range_squared: f64,
}

impl Components {
    pub fn new(material_data_vector: Vec<MaterialData>, parts_vector: Vec<PartTypes>) -> Self {
        // Has to be updated after creation of the Components-code.
        let simulation_range_squared = -1.0;

        let material_property = MaterialProperties::default();
        let mut cached_material_properties: Vec<MaterialProperties> = Vec::new();

        for _ in material_data_vector.iter() {
            cached_material_properties.push(material_property.clone());
        }

        let is_cache_initialized = false;

        Components {
            material_data_vector,
            parts_vector,
            cached_material_properties,
            cache_initialized: is_cache_initialized,
            simulation_range_squared,
        }
    }

    /// Automatically calculates the maximum radius squared, beyond which the neutron is discarded.
    /// If this wasn't set correctly manually, it would mess up (if it's too small, part of the geometry would be ignored; too large, and the simulation becomes very slow if neutrons have to escape first).
    /// The code iterates over all the parts, skipping those with order <= -1, and determines the largest bounds.
    /// This function should be ran after creation of the simulation to set components.simulation_range_squared.
    pub fn get_maximum_radius_squared(&mut self) {
        let mut maximum_radius = 0.0;

        for part in &self.parts_vector {
            let (bounding_box, center, order) = match part {
                PartTypes::Sphere(sphere) => (&sphere.bounding_box, &sphere.center, &sphere.order),
                PartTypes::Cylinder(cylinder) => {
                    (&cylinder.bounding_box, &cylinder.center, &cylinder.order)
                }
                PartTypes::Cuboid(cuboid) => (&cuboid.bounding_box, &cuboid.center, &cuboid.order),
            };

            // To allow a large background using for example a cube (computationally efficient because no squaring for the radius), without having it included in the simulation range, if the order is specified as -1 or less, it will be skipped.
            if order <= &-1 {
                continue;
            }

            let coordinate_min_radius = center.add(bounding_box.min).norm_squared();
            let coordinate_max_radius = center.add(bounding_box.max).norm_squared();

            if coordinate_max_radius > maximum_radius {
                maximum_radius = coordinate_max_radius
            }
            if coordinate_min_radius > maximum_radius {
                maximum_radius = coordinate_min_radius
            }
        }

        debug!("Maximum part radius: {:.3} m.", maximum_radius.sqrt());

        self.simulation_range_squared = maximum_radius;
    }

    /// Updating the cache of material properties for the given neutron's energy.
    /// This should be done any time the neutron's energy changes significantly, or whenever the simulation starts.
    pub fn update_cache_properties(&mut self, neutron_energy: f64) {
        for (index, material_data) in self.material_data_vector.iter().enumerate() {
            self.cached_material_properties[index].get_properties(material_data, neutron_energy);
        }
        self.cache_initialized = true;
    }

    /// Calculates the composition's total cross-section for a given neutron's energy from the cached properties.
    /// The total cross-section can then be used in the neutron dynamics to calculate interaction probabilities.
    /// If the cache is not updated, this will mess up.
    pub fn get_composition_total_cross_section(
        &self,
        part_composition_vector: &Vec<PartComposition>,
    ) -> f64 {
        let mut overall_total_cross_section: f64 = 0.0;

        debug_assert!(self.cache_initialized, "Cache was not initialized!");

        for part_composition in part_composition_vector {
            let material_name = part_composition.material_name;
            let material_index = map_enum_to_indices(&material_name);
            let material_composition = &self.cached_material_properties[material_index];

            overall_total_cross_section +=
                material_composition.total_cross_section() * part_composition.material_fraction;

            // debug!(
            //     "In the function: Material cross section for {:?}: {}",
            //     material_name,
            //     material_composition.total_cross_section()
            // );

            // debug!(
            //     "Material: {:?} - {}",
            //     material_name,
            //     material_composition.total_cross_section()
            // );
        }

        // debug!("Overall cross-section: {}", overall_total_cross_section);

        overall_total_cross_section
    }

    /// Sums each of the part's material composition vectors and ensures the fractions add up to 1. If not, it throws an error.
    /// This is probably a very error-prone operation, if we specify the parts manually.
    /// If we move to configuration files later, we can move this to integration tests.
    pub fn check_material_fractions_sum(&self) {
        for part in &self.parts_vector {
            let material_composition_vector = match part {
                PartTypes::Sphere(sphere) => &sphere.material_composition_vector,
                PartTypes::Cylinder(cylinder) => &cylinder.material_composition_vector,
                PartTypes::Cuboid(cuboid) => &cuboid.material_composition_vector,
            };

            let mut material_sum = 0.0;

            for material_composition in material_composition_vector {
                material_sum += material_composition.material_fraction;
            }

            assert!(
                material_sum == 1.0,
                "The material composition summed up to {:?} instead of 1.0 for part:\n{:?}",
                material_sum,
                part,
            );
        }
    }

    /// Determines which material from a given composition interacts with the neutron.
    /// This is mainly relevant for mixed materials, such as water (H-1/O-16), U-235/U-238 etc.
    pub fn select_material_from_composition(
        &self,
        rng: &mut rand::rngs::SmallRng,
        part_composition_vector: &Vec<PartComposition>,
        composition_total_cross_section: f64,
    ) -> usize {
        let material_selection_criterion = rng.gen::<f64>();
        let mut cumulative_probability = 0.0;

        debug_assert!(self.cache_initialized, "Cache was not initialized!");

        // debug!("Criterion: {}", material_selection_criterion);

        for part_composition in part_composition_vector.iter() {
            let material_name = part_composition.material_name;
            let material_index = map_enum_to_indices(&material_name);
            let material_composition = &self.cached_material_properties[material_index];
            let normalized_cross_section = material_composition.total_cross_section()
                * part_composition.material_fraction
                / composition_total_cross_section;

            // debug!(
            //     "Material cross section for {:?}: {}. After normalization: {}",
            //     material_name,
            //     material_composition.total_cross_section(),
            //     normalized_cross_section,
            // );

            // debug!(
            //     "Normalized cross-section {} for material {:?}",
            //     normalized_cross_section, material_name
            // );

            // debug!("Cumulative probability: {}", cumulative_probability);

            if material_selection_criterion >= cumulative_probability
                && material_selection_criterion < cumulative_probability + normalized_cross_section
            {
                let cached_material_vector_index = map_enum_to_indices(&material_name);

                // debug!(
                //     "Material: {:?}",
                //     self.cached_material_properties[cached_material_vector_index].name
                // );

                return cached_material_vector_index;
            }

            cumulative_probability += normalized_cross_section;
        }

        // debug!("Total cross sections: {}", composition_total_cross_section);

        // The RNG generates \xi \in [0, 1), and we check 0 <= \xi x_i, with x_i the material fraction.
        // If something goes wrong, or if we are outside any material (i.e. when the specified material is Void), we return 0.
        0
    }

    /// Gets the material index based on the neutron's current position by checking each individual part and their order.
    /// The part with the highest order is selected, through constructive solid geometry.
    /// The actual isotope that is interacted with is then selected from a composite material and returned, together with the total cross-section.
    pub fn get_material_index(
        &self,
        rng: &mut rand::rngs::SmallRng,
        neutron_position: &Vec3D,
    ) -> (usize, f64) {
        let mut maximum_order = i32::MIN;

        let empty_reference_vector: Vec<PartComposition> = Vec::default();
        let mut max_part_composition_vector: &Vec<PartComposition> = &empty_reference_vector;

        // Iterates over all the different parts.
        for part in &self.parts_vector {
            // Checks each option in the enum.
            let (is_inside, order, material_composition_vector) = match part {
                PartTypes::Sphere(sphere) => (
                    sphere.is_inside(neutron_position),
                    sphere.order,
                    &sphere.material_composition_vector,
                ),
                PartTypes::Cylinder(cylinder) => (
                    cylinder.is_inside(neutron_position),
                    cylinder.order,
                    &cylinder.material_composition_vector,
                ),
                PartTypes::Cuboid(cuboid) => (
                    cuboid.is_inside(neutron_position),
                    cuboid.order,
                    &cuboid.material_composition_vector,
                ),
            };

            if order > maximum_order && is_inside == true {
                maximum_order = order;
                max_part_composition_vector = material_composition_vector;

                // for material_composition in max_part_composition_vector.iter() {
                //     debug!(
                //         "Looking at {:?}. Inside: {}, with order {}",
                //         material_composition.material_name, is_inside, order
                //     );
                // }
            }
        }

        let composition_total_cross_section =
            self.get_composition_total_cross_section(max_part_composition_vector);

        // debug!("{}", composition_total_cross_section);

        let max_material_index = self.select_material_from_composition(
            rng,
            max_part_composition_vector,
            composition_total_cross_section,
        );

        // debug!("Max material index: {}", max_material_index);

        // let material_name = map_indices_to_enum(max_material_index);
        // info!("Material name: {:?}", max_material_index);

        (max_material_index, composition_total_cross_section)
    }

    /// Gets the material properties and total cross-section based on the neutron's current position.
    /// This requires the cache to have been updated beforehand.
    /// The function will throw an exception if this has not been done.
    pub fn get_material_properties(
        &self,
        rng: &mut rand::rngs::SmallRng,
        neutron_position: &Vec3D,
    ) -> (&MaterialProperties, f64) {
        // Ensuring everything is correctly initialized.
        debug_assert!(self.cache_initialized, "Cache was not initialized!");
        debug_assert!(
            self.simulation_range_squared > 0.0,
            "Simulation range is not set correctly."
        );

        let (material_index, composition_total_cross_section) =
            self.get_material_index(rng, neutron_position);
        let material_properties = &self.cached_material_properties[material_index];

        (material_properties, composition_total_cross_section)
    }
}
