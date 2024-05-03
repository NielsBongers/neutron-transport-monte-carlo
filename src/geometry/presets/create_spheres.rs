use crate::geometry::parts::parts::PartTypes;
use crate::geometry::parts::sphere::Sphere;
use crate::materials::material_properties::MaterialNames;
use crate::utils::vectors::Vec3D;

use crate::geometry::components::PartComposition;

pub fn create_sphere(
    center: Vec3D,
    radius: f64,
    material_composition_vector: Vec<PartComposition>,
    order: i32,
) -> Vec<PartTypes> {
    let sphere = Sphere::new(
        center,
        radius,
        MaterialNames::U235,
        material_composition_vector,
        order,
    );
    return vec![PartTypes::Sphere(sphere)];
}

pub fn create_default_sphere(radius: f64) -> Vec<PartTypes> {
    let center = Vec3D {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    let order: i32 = 1;

    let u235_composition = PartComposition {
        material_name: MaterialNames::U235,
        material_fraction: 1.0,
    };
    // let u238_composition = PartComposition {
    //     material_name: MaterialNames::U238,
    //     material_fraction: 0.06,
    // };

    let material_composition_vector = vec![u235_composition];

    create_sphere(center, radius, material_composition_vector, order)
}

/// Equivalent to Godiva reference sphere.
/// Based on Burgio2004, "Time resolved MCNP neutron transport simulation on multiplying media: GODIVA benchmark"
/// 94% U-235, with k = 1 at r = 0.087037.
pub fn create_reference_sphere() -> Vec<PartTypes> {
    let center = Vec3D {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    let radius: f64 = 0.087037;
    let order: i32 = 1;

    let u235_composition = PartComposition {
        material_name: MaterialNames::U235,
        material_fraction: 0.94,
    };
    let u238_composition = PartComposition {
        material_name: MaterialNames::U238,
        material_fraction: 0.06,
    };

    let material_composition_vector = vec![u238_composition, u235_composition];

    return create_sphere(center, radius, material_composition_vector, order);
}

pub fn create_water_body() -> Vec<PartTypes> {
    let center = Vec3D {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    let radius = 100.0;
    let order = 1;

    let hydrogen_in_water = PartComposition {
        material_name: MaterialNames::H1,
        material_fraction: 2. / 3.,
    };

    let oxygen_in_water = PartComposition {
        material_name: MaterialNames::O16,
        material_fraction: 1. / 3.,
    };

    let material_composition_vector = vec![hydrogen_in_water, oxygen_in_water];

    return create_sphere(center, radius, material_composition_vector, order);
}
