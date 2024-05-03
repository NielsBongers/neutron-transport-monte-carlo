use log::debug;
use nuclear;
use nuclear::geometry::components::Components;
use nuclear::geometry::components::PartComposition;
use nuclear::geometry::parts::cuboid::Cuboid;
use nuclear::geometry::parts::cylinder::Cylinder;
use nuclear::geometry::parts::sphere::Sphere;
use nuclear::geometry::presets::create_spheres::create_reference_sphere;
use nuclear::materials::material_properties::{get_material_data_vector, MaterialNames};
use nuclear::materials::material_properties::{map_enum_to_indices, map_indices_to_enum};
use nuclear::utils::vectors::Vec3D;

#[test]
fn check_bounding_boxes() {
    let center = Vec3D {
        x: 1.0,
        y: 2.0,
        z: 3.0,
    };
    let direction: Vec3D = Vec3D {
        x: 1.0,
        y: 0.0,
        z: 0.0,
    };

    let radius = 0.5;
    let length = 1.5;
    let depth = 2.0;
    let height = 2.0;

    let material_name: MaterialNames = MaterialNames::U235;
    let u235_composition = PartComposition {
        material_name: MaterialNames::U235,
        material_fraction: 1.0,
    };
    let material_composition_vector = vec![u235_composition];
    let order = 1;

    // Defining the geometries.
    let sphere: Sphere = Sphere::new(
        center,
        radius,
        material_name,
        material_composition_vector.clone(),
        order,
    );

    let cylinder: Cylinder = Cylinder::new(
        center,
        direction,
        length,
        radius,
        material_name,
        material_composition_vector.clone(),
        order,
    );

    let cuboid: Cuboid = Cuboid::new(
        center,
        length,
        depth,
        height,
        material_name,
        material_composition_vector,
        order,
    );

    // Defining various neutron positions to test with.
    let neutron_at_origin: Vec3D = Vec3D {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    let neutron_edge_sphere: Vec3D = Vec3D {
        x: sphere.center.x + sphere.radius,
        y: sphere.center.y,
        z: sphere.center.z,
    };

    let neutron_left_edge_cylinder: Vec3D = Vec3D {
        x: cylinder.center.x - cylinder.length / 2.0,
        y: cylinder.center.y,
        z: cylinder.center.z,
    };

    let neutron_right_edge_cylinder: Vec3D = Vec3D {
        x: cylinder.center.x + cylinder.length / 2.0,
        y: cylinder.center.y,
        z: cylinder.center.z,
    };

    let neutron_center_edge_cylinder: Vec3D = Vec3D {
        x: cylinder.center.x,
        y: cylinder.center.y + cylinder.radius,
        z: cylinder.center.z,
    };

    let neutron_center_edge_bbox_cylinder: Vec3D = Vec3D {
        x: cylinder.center.x,
        y: cylinder.center.y + cylinder.radius,
        z: cylinder.center.z + cylinder.radius,
    };

    let neutron_top_right_cuboid: Vec3D = Vec3D {
        x: cuboid.center.x + length / 2.0,
        y: cuboid.center.y + depth / 2.0,
        z: cuboid.center.z + height / 2.0,
    };

    let neutron_top_right_outside_cuboid: Vec3D = Vec3D {
        x: cuboid.center.x + length,
        y: cuboid.center.y + depth,
        z: cuboid.center.z + height,
    };

    // Assertions for the sphere.
    assert!(!sphere.is_inside_bounding_box(&neutron_at_origin));
    assert!(sphere.is_inside_bounding_box(&neutron_edge_sphere));
    assert!(sphere.is_inside_bounding_box(&center));
    assert!(!sphere.is_inside(&neutron_at_origin));
    assert!(!sphere.is_inside(&sphere.bounding_box.min));
    assert!(!sphere.is_inside(&sphere.bounding_box.max));
    assert!(sphere.is_inside(&neutron_edge_sphere));
    assert!(sphere.is_inside(&center));

    // Assertions for the cylinder.
    assert!(cylinder.is_inside_bounding_box(&center));
    assert!(cylinder.is_inside_bounding_box(&neutron_right_edge_cylinder));
    assert!(cylinder.is_inside_bounding_box(&neutron_left_edge_cylinder));
    assert!(cylinder.is_inside_bounding_box(&neutron_center_edge_cylinder));
    assert!(cylinder.is_inside_bounding_box(&neutron_center_edge_bbox_cylinder));

    assert!(cylinder.is_inside(&center));
    assert!(cylinder.is_inside(&neutron_right_edge_cylinder));
    assert!(cylinder.is_inside(&neutron_left_edge_cylinder));
    assert!(cylinder.is_inside(&neutron_center_edge_cylinder));
    assert!(!cylinder.is_inside(&neutron_center_edge_bbox_cylinder));

    // Assertions for the cuboid.
    assert!(!cuboid.is_inside_bounding_box(&neutron_at_origin));
    assert!(cuboid.is_inside_bounding_box(&center));
    assert!(cuboid.is_inside_bounding_box(&neutron_top_right_cuboid));
    assert!(!cuboid.is_inside_bounding_box(&neutron_top_right_outside_cuboid));

    assert!(!cuboid.is_inside(&neutron_at_origin));
    assert!(cuboid.is_inside(&center));
    assert!(cuboid.is_inside(&neutron_top_right_cuboid));
    assert!(!cuboid.is_inside(&neutron_top_right_outside_cuboid));
}

#[test]
fn check_material_indices() {
    let mut components: Components =
        Components::new(get_material_data_vector(), create_reference_sphere());
    components.update_cache_properties(1e6);

    for (vector_index, material_data) in components.material_data_vector.iter().enumerate() {
        let material_name_in_vector = material_data.name;
        let material_name_in_index_to_enum = map_indices_to_enum(vector_index);
        let index_in_enum_to_index = map_enum_to_indices(&material_name_in_vector);

        debug!(
            "Vector index: {}, Vector name: {:?}, index name: {:?}, enum index: {}",
            vector_index,
            material_name_in_vector,
            material_name_in_index_to_enum,
            index_in_enum_to_index
        );

        assert_eq!(material_name_in_index_to_enum, material_name_in_vector);
        assert_eq!(vector_index, index_in_enum_to_index);
    }

    for (vector_index, material_data) in components.cached_material_properties.iter().enumerate() {
        let material_name_in_vector = material_data.name;
        let material_name_in_index_to_enum = map_indices_to_enum(vector_index);
        let index_in_enum_to_index = map_enum_to_indices(&material_name_in_vector);

        assert_eq!(material_name_in_index_to_enum, material_name_in_vector);
        assert_eq!(vector_index, index_in_enum_to_index);
    }
}
