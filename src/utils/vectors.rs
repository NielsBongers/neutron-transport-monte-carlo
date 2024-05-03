use rand::Rng;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct Vec3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Default for Vec3D {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
}

impl fmt::Display for Vec3D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl Vec3D {
    pub fn norm(&self) -> f64 {
        f64::sqrt(self.x * self.x + self.y * self.y + self.z * self.z)
    }

    /// Faster implementation that doesn't use a square root.
    pub fn norm_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn to_unit_vec(&mut self) -> () {
        let norm = self.norm();
        self.x = self.x / norm;
        self.y = self.y / norm;
        self.z = self.z / norm;
    }

    pub fn add(&self, second_vector: Vec3D) -> Vec3D {
        Vec3D {
            x: self.x + second_vector.x,
            y: self.y + second_vector.y,
            z: self.z + second_vector.z,
        }
    }

    pub fn subtract(&self, second_vector: Vec3D) -> Vec3D {
        Vec3D {
            x: self.x - second_vector.x,
            y: self.y - second_vector.y,
            z: self.z - second_vector.z,
        }
    }

    pub fn scalar_add(&self, scalar: f64) -> Vec3D {
        Vec3D {
            x: self.x + scalar,
            y: self.y + scalar,
            z: self.z + scalar,
        }
    }

    pub fn dot(&self, second_vector: Vec3D) -> f64 {
        self.x * second_vector.x + self.y * second_vector.y + self.z * second_vector.z
    }

    pub fn scalar_dot(&self, scalar: f64) -> Vec3D {
        Vec3D {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }

    pub fn min(&self, second_vector: Vec3D) -> Vec3D {
        Vec3D {
            x: self.x.min(second_vector.x),
            y: self.y.min(second_vector.y),
            z: self.z.min(second_vector.z),
        }
    }

    pub fn max(&self, second_vector: Vec3D) -> Vec3D {
        Vec3D {
            x: self.x.max(second_vector.x),
            y: self.y.max(second_vector.y),
            z: self.z.max(second_vector.z),
        }
    }

    pub fn random_unit_vector(rng: &mut rand::rngs::SmallRng) -> Vec3D {
        let mut new_direction: Vec3D = Vec3D {
            x: 1.0 - rng.gen::<f64>() * 2.0,
            y: 1.0 - rng.gen::<f64>() * 2.0,
            z: 1.0 - rng.gen::<f64>() * 2.0,
        };
        new_direction.to_unit_vec();

        new_direction
    }
}
