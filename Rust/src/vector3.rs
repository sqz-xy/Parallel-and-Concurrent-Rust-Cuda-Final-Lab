use std::ops::{Add, Sub, Mul};
use std::fmt::{self, Display};

#[derive(Debug, Copy, Clone)]
pub struct Vector3f {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl Vector3f {
    pub fn new (p_x: f32, p_y: f32, p_z: f32) -> Vector3f {
        Vector3f {
            x: p_x,
            y: p_y,
            z: p_z
        }
    }

    pub fn empty() -> Vector3f {
        Vector3f {
            x: 0.0,
            y: 0.0,
            z: 0.0
        }
    }

    pub fn magnitude(&self) -> f32 {
        return f32::sqrt((self.x * self.x) + (self.y * self.y) + (self.z * self.z));
    }

    pub fn normalise(&mut self) {
        self.x /= self.magnitude();
        self.y /= self.magnitude();
        self.z /= self.magnitude();
    }

    pub fn multiply_f32(&mut self, rhs: f32) {
            self.x *= rhs;
            self.y *= rhs;
            self.z *= rhs;
    }

    pub fn add_f32(&mut self, rhs: f32) {
            self.x += rhs;
            self.y += rhs;
            self.z += rhs;
    }

    pub fn sub_f32(&mut self, rhs: f32) {
            self.x -= rhs;
            self.y -= rhs;
            self.z -= rhs;
    }
}

// Override add operator
impl Add for Vector3f {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        }
    }
}

// Override sub operator
impl Sub for Vector3f {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z
        }
    }
}

impl Mul for Vector3f {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z
        }   
    }
}

// Override output
impl Display for Vector3f {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x: {}, y: {}, z: {}", self.x, self.y, self.z)
    }
}