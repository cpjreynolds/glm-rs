use ::{
    Dot,
    Cross,
    Length,
    Normalize,
};

use std::mem;

use std::ops::{
    Add,
    Sub,
    Mul,
    Div,
};

use num::{
    One,
    Zero,
};

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 {
            x: x,
            y: y,
            z: z,
        }
    }

    pub fn as_array(&self) -> &[f32; 3] {
        unsafe {
            mem::transmute(self)
        }
    }

    pub fn as_array_mut(&mut self) -> &mut [f32; 3] {
        unsafe {
            mem::transmute(self)
        }
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Add<f32> for Vec3 {
    type Output = Self;

    fn add(self, rhs: f32) -> Self::Output {
        Vec3 {
            x: self.x + rhs,
            y: self.y + rhs,
            z: self.z + rhs,
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Sub<f32> for Vec3 {
    type Output = Self;

    fn sub(self, rhs: f32) -> Self::Output {
        Vec3 {
            x: self.x - rhs,
            y: self.y - rhs,
            z: self.z - rhs,
        }
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Vec3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl Dot for Vec3 {
    fn dot(&self, rhs: &Self) -> f32 {
        (self.x * rhs.x) + (self.y * rhs.y) + (self.z * rhs.z)
    }
}

impl Cross for Vec3 {
    fn cross(&self, rhs: &Self) -> Self {
        Vec3::new(self.y * rhs.z - rhs.y * self.z,
                  self.z * rhs.x - rhs.z * self.x,
                  self.x * rhs.y - rhs.x * self.y,)
    }
}

impl Normalize for Vec3 {
    fn normalize(&self) -> Self {
        *self * (f32::one() / f32::sqrt(self.dot(self)))
    }
}

impl Length for Vec3 {
    fn length(&self) -> f32 {
        f32::sqrt(self.dot(self))
    }
}

impl Zero for Vec3 {
    fn zero() -> Self {
        Vec3 {
            x: f32::zero(),
            y: f32::zero(),
            z: f32::zero(),
        }
    }

    fn is_zero(&self) -> bool {
        *self == Self::zero()
    }
}

