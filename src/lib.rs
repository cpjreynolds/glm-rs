extern crate num;

mod vector;
mod matrix;

pub use vector::Vec3;
pub use matrix::Mat4;

pub fn radians(degrees: f32) -> f32 {
    degrees * (::std::f32::consts::PI / 180.0f32)
}

pub fn degrees(radians: f32) -> f32 {
    radians * (180.0f32 / ::std::f32::consts::PI)
}

pub trait Dot {
    fn dot(&self, rhs: &Self) -> f32;
}

pub trait Cross {
    fn cross(&self, rhs: &Self) -> Self;
}

pub trait Length {
    fn length(&self) -> f32;
}

pub trait SqLength {


pub trait Normalize {
    fn normalize(&self) -> Self;
}

pub trait Eye {
    fn eye() -> Self;
}

