use serde::{Deserialize, Serialize};

use crate::geometry::parts::cuboid::Cuboid;
use crate::geometry::parts::cylinder::Cylinder;
use crate::geometry::parts::sphere::Sphere;

#[derive(Debug, Serialize, Deserialize)]
pub enum PartTypes {
    Cylinder(Cylinder),
    Sphere(Sphere),
    Cuboid(Cuboid),
}
