use std::fs;

use crate::geometry::components::PartComposition;
use crate::geometry::parts::cuboid::Cuboid;
use crate::geometry::parts::cylinder::Cylinder;
use crate::geometry::parts::parts::PartTypes;
use crate::geometry::parts::sphere::Sphere;
use crate::materials::material_properties::MaterialNames;
use crate::utils::vectors::Vec3D;
use serde::Deserialize;

/// Loading in data for cylinders from a TOML.
#[derive(Deserialize, Debug)]
pub struct CylinderTOML {
    pub center: Vec3D,
    pub direction: Vec3D,
    pub length: f64,
    pub radius: f64,
    pub material_name: MaterialNames,
    pub material_composition_vector: Vec<PartComposition>,
    pub order: i32,
}

/// Loading in data for cuboids from a TOML.
#[derive(Deserialize, Debug)]
pub struct CuboidTOML {
    pub center: Vec3D,
    pub width: f64,
    pub depth: f64,
    pub height: f64,
    pub material_name: MaterialNames,
    pub material_composition_vector: Vec<PartComposition>,
    pub order: i32,
}

/// Loading in data for spheres from a TOML.
#[derive(Deserialize, Debug)]
pub struct SphereTOML {
    pub center: Vec3D,
    pub radius: f64,
    pub material_name: MaterialNames,
    pub material_composition_vector: Vec<PartComposition>,
    pub order: i32,
}

/// Combining all the data for spheres, cuboids and cylinders into a single struct for serde.
#[derive(Deserialize, Debug)]
struct Objects {
    spheres: Option<Vec<SphereTOML>>,
    cuboids: Option<Vec<CuboidTOML>>,
    cylinders: Option<Vec<CylinderTOML>>,
}

/// Loading geometries from a specified TOML path into a vector, which can then be read by the simulation.
pub fn load_geometries(toml_path: &String) -> Vec<PartTypes> {
    let toml_str = fs::read_to_string(toml_path).expect("Failed to read geometries TOML.");
    let objects: Objects =
        toml::from_str(&toml_str).expect("Failed to parse object from TOML string using serde.");

    let mut parts_vector = Vec::<PartTypes>::default();

    if let Some(spheres) = objects.spheres {
        for toml_sphere in spheres {
            let sphere = Sphere::new(
                toml_sphere.center,
                toml_sphere.radius,
                toml_sphere.material_name,
                toml_sphere.material_composition_vector,
                toml_sphere.order,
            );
            parts_vector.push(PartTypes::Sphere(sphere));
        }
    }

    if let Some(cuboids) = objects.cuboids {
        for toml_cuboid in cuboids {
            let cuboid = Cuboid::new(
                toml_cuboid.center,
                toml_cuboid.width,
                toml_cuboid.depth,
                toml_cuboid.height,
                toml_cuboid.material_name,
                toml_cuboid.material_composition_vector,
                toml_cuboid.order,
            );
            parts_vector.push(PartTypes::Cuboid(cuboid));
        }
    }

    if let Some(cylinders) = objects.cylinders {
        for toml_cylinder in cylinders {
            let cylinder = Cylinder::new(
                toml_cylinder.center,
                toml_cylinder.direction,
                toml_cylinder.length,
                toml_cylinder.radius,
                toml_cylinder.material_name,
                toml_cylinder.material_composition_vector,
                toml_cylinder.order,
            );
            parts_vector.push(PartTypes::Cylinder(cylinder));
        }
    }

    parts_vector
}
