use crate::geometry::parts::cuboid::Cuboid;
use crate::geometry::parts::parts::PartTypes;
use crate::materials::material_properties::MaterialNames;
use crate::utils::vectors::Vec3D;
use log::debug;

use crate::geometry::components::PartComposition;

pub fn create_plate_reactor(plate_thickness: f64) -> Vec<PartTypes> {
    let u235_composition = PartComposition {
        material_name: MaterialNames::U235,
        material_fraction: 0.94,
    };
    let u238_composition = PartComposition {
        material_name: MaterialNames::U238,
        material_fraction: 0.06,
    };

    let uranium_fuel = vec![u238_composition, u235_composition];

    let hydrogen_in_water = PartComposition {
        material_name: MaterialNames::H1,
        material_fraction: 2. / 3.,
    };

    let oxygen_in_water = PartComposition {
        material_name: MaterialNames::O16,
        material_fraction: 1. / 3.,
    };

    let water = vec![hydrogen_in_water, oxygen_in_water];

    let first_plate_center = Vec3D {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    // let plate_thickness = 0.035;
    let plate_height = 0.50;
    let plate_width = 0.50;

    let plate_separation = 0.10;
    let plate_count = 5;

    let background_water_size = 10.0;

    let mut reactor_parts: Vec<PartTypes> = Vec::new();

    let water_background = Cuboid::new(
        first_plate_center,
        background_water_size,
        background_water_size,
        background_water_size,
        MaterialNames::H1,
        water,
        -1,
    );

    reactor_parts.push(PartTypes::Cuboid(water_background));

    for i in 0..plate_count {
        let plate_x_position =
            first_plate_center.x + i as f64 * (2.0 * plate_thickness / 2.0 + plate_separation);

        let plate_center = Vec3D {
            x: plate_x_position,
            y: first_plate_center.y,
            z: first_plate_center.z,
        };

        let plate = Cuboid::new(
            plate_center,
            plate_thickness,
            plate_width,
            plate_height,
            MaterialNames::U235,
            uranium_fuel.clone(),
            1,
        );

        reactor_parts.push(PartTypes::Cuboid(plate));

        debug!(
            "Currently at plate {}. Plate x position: {}",
            i, plate_x_position
        );
    }

    reactor_parts
}
