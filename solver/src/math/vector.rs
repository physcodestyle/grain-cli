use std::fmt;

#[derive(Clone, Copy)]
pub struct Vec2d {
    pub x: f64,
    pub y: f64,
}

#[derive(Clone, Copy)]
pub struct Vec2dPolar {
    pub alpha: f64, // Radians
    pub r: f64,
}

impl fmt::Display for Vec2d {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}\t{}", self.x, self.y)
    }
}

impl Vec2d {
    pub fn new(x: f64, y: f64) -> Vec2d {
        Vec2d { x: x, y: y }
    }

    pub fn add(&self, other: &Vec2d) -> Vec2d {
        Vec2d {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    pub fn add_eq(&mut self, other: &Vec2d) {
        self.x += other.x;
        self.y += other.y;
    }

    pub fn equal(&self, other: &Vec2d, accuracy: f64) -> bool {
        (self.x - other.x).abs() < accuracy && (self.y - other.y).abs() < accuracy
    }

    pub fn length(&self) -> f64 {
        (self.x.powf(2.0) + self.y.powf(2.0)).sqrt()
    }

    pub fn mul(&self, scalar: &f64) -> Vec2d {
        Vec2d {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }

    pub fn mul_eq(&mut self, scalar: &f64) {
        self.x *= scalar;
        self.y *= scalar;
    }

    // Scalar Product
    pub fn scp(&self, other: &Vec2d) -> f64 {
        self.x * other.x + self.y * other.y
    }

    pub fn sub(&self, other: &Vec2d) -> Vec2d {
        Vec2d {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }

    pub fn sub_eq(&mut self, other: &Vec2d) {
        self.x -= other.x;
        self.y -= other.y;
    }

    pub fn to_polar(&self) -> Vec2dPolar {
        Vec2dPolar {
            alpha: (self.y / self.x).atan(), // Radians
            r: (self.x.powf(2.0) + self.y.powf(2.0)).sqrt(),
        }
    }
}

#[derive(Clone, Copy)]
pub struct Vec3d {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Clone, Copy)]
pub struct Vec3dPolar {
    pub alpha: f64, // Radians
    pub r: f64,
    pub z: f64,
}

#[derive(Clone, Copy)]
pub struct Vec3dSpherical {
    pub alpha: f64, // Radians
    pub beta: f64,  // Radians
    pub r: f64,
}

impl fmt::Display for Vec3d {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}\t{}\t{}", self.x, self.y, self.z)
    }
}

impl Vec3d {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3d {
        Vec3d { x: x, y: y, z: z }
    }

    pub fn add(&self, other: &Vec3d) -> Vec3d {
        Vec3d {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }

    pub fn add_eq(&mut self, other: &Vec3d) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }

    pub fn equal(&self, other: &Vec3d, accuracy: f64) -> bool {
        (self.x - other.x).abs() < accuracy
            && (self.y - other.y).abs() < accuracy
            && (self.z - other.z).abs() < accuracy
    }

    pub fn length(&self) -> f64 {
        (self.x.powf(2.0) + self.y.powf(2.0) + self.z.powf(2.0)).sqrt()
    }

    pub fn mul(&self, scalar: &f64) -> Vec3d {
        Vec3d {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }

    pub fn mul_eq(&mut self, scalar: &f64) {
        self.x *= scalar;
        self.y *= scalar;
        self.z *= scalar;
    }

    // Scalar Product
    pub fn scp(&self, other: &Vec3d) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn sub(&self, other: &Vec3d) -> Vec3d {
        Vec3d {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }

    pub fn sub_eq(&mut self, other: &Vec3d) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }

    pub fn to_polar(&self) -> Vec3dPolar {
        Vec3dPolar {
            alpha: (self.y / self.x).atan(), // Radians
            r: (self.x.powf(2.0) + self.y.powf(2.0) + self.z.powf(2.0)).sqrt(),
            z: self.z,
        }
    }

    pub fn to_spherical(&self) -> Vec3dSpherical {
        Vec3dSpherical {
            alpha: (self.y / self.x).atan(),
            beta: (self.z / (self.x.powf(2.0) + self.y.powf(2.0)).sqrt()).atan(),
            r: (self.x.powf(2.0) + self.y.powf(2.0) + self.z.powf(2.0)).sqrt(),
        }
    }

    // Vector Product
    pub fn vep(&self, other: &Vec3d) -> Vec3d {
        Vec3d {
            x: self.y * other.z + self.z * other.y,
            y: self.z * other.x + self.x * other.z,
            z: self.x * other.y + self.y * other.x,
        }
    }
}
