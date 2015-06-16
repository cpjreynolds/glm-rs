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
pub struct Mat4 {
    pub m11: f32, pub m21: f32, pub m31: f32, pub m41: f32,
    pub m12: f32, pub m22: f32, pub m32: f32, pub m42: f32,
    pub m13: f32, pub m23: f32, pub m33: f32, pub m43: f32,
    pub m14: f32, pub m24: f32, pub m34: f32, pub m44: f32
}

impl Mat4 {
    pub fn new(m11: f32, m12: f32, m13: f32, m14: f32,
               m21: f32, m22: f32, m23: f32, m24: f32,
               m31: f32, m32: f32, m33: f32, m34: f32,
               m41: f32, m42: f32, m43: f32, m44: f32) -> Mat4 {
        Mat4 {
            m11: m11, m21: m21, m31: m31, m41: m41,
            m12: m12, m22: m22, m32: m32, m42: m42,
            m13: m13, m23: m23, m33: m33, m43: m43,
            m14: m14, m24: m24, m34: m34, m44: m44
        }
    }

    pub fn as_array(&self) -> &[[f32; 4]; 4] {
        unsafe {
            mem::transmute(self)
        }
    }

    pub fn as_array_mut(&mut self) -> &mut [[f32; 4]; 4] {
        unsafe {
            mem::transmute(self)
        }
    }
}


