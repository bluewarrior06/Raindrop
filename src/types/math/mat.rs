use std::ops::Mul;

use crate::types::{
    math::vec::{Vec4D, Vec4Df},
    pod::pmat::pMat4x4f,
};

// Matrix 4x4
pub type Mat4x4f = Mat4x4<f32>;

#[derive(Debug, Default, Clone, Copy)]
pub struct Mat4x4<T> {
    pub x: Vec4D<T>,
    pub y: Vec4D<T>,
    pub z: Vec4D<T>,
    pub w: Vec4D<T>,
}

impl Into<pMat4x4f> for Mat4x4f {
    fn into(self) -> pMat4x4f {
        pMat4x4f {
            x: self.x.into(),
            y: self.y.into(),
            z: self.z.into(),
            w: self.w.into(),
        }
    }
}

impl Mul for Mat4x4f {
    type Output = Mat4x4f;
    fn mul(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: Vec4Df::new(
                self.x.x * rhs.x.x + self.x.y * rhs.y.x + self.x.z * rhs.z.x + self.x.w * rhs.w.x,
                self.x.x * rhs.x.y + self.x.y * rhs.y.y + self.x.z * rhs.z.y + self.x.w * rhs.w.y,
                self.x.x * rhs.x.z + self.x.y * rhs.y.z + self.x.z * rhs.z.z + self.x.w * rhs.w.z,
                self.x.x * rhs.x.w + self.x.y * rhs.y.w + self.x.z * rhs.z.w + self.x.w * rhs.w.w,
            ),
            y: Vec4Df::new(
                self.y.x * rhs.x.x + self.y.y * rhs.y.x + self.y.z * rhs.z.x + self.y.w * rhs.w.x,
                self.y.x * rhs.x.y + self.y.y * rhs.y.y + self.y.z * rhs.z.y + self.y.w * rhs.w.y,
                self.y.x * rhs.x.z + self.y.y * rhs.y.z + self.y.z * rhs.z.z + self.y.w * rhs.w.z,
                self.y.x * rhs.x.w + self.y.y * rhs.y.w + self.y.z * rhs.z.w + self.y.w * rhs.w.w,
            ),
            z: Vec4Df::new(
                self.z.x * rhs.x.x + self.z.y * rhs.y.x + self.z.z * rhs.z.x + self.z.w * rhs.w.x,
                self.z.x * rhs.x.y + self.z.y * rhs.y.y + self.z.z * rhs.z.y + self.z.w * rhs.w.y,
                self.z.x * rhs.x.z + self.z.y * rhs.y.z + self.z.z * rhs.z.z + self.z.w * rhs.w.z,
                self.z.x * rhs.x.w + self.z.y * rhs.y.w + self.z.z * rhs.z.w + self.z.w * rhs.w.w,
            ),
            w: Vec4Df::new(
                self.w.x * rhs.x.x + self.w.y * rhs.y.x + self.w.z * rhs.z.x + self.w.w * rhs.w.x,
                self.w.x * rhs.x.y + self.w.y * rhs.y.y + self.w.z * rhs.z.y + self.w.w * rhs.w.y,
                self.w.x * rhs.x.z + self.w.y * rhs.y.z + self.w.z * rhs.z.z + self.w.w * rhs.w.z,
                self.w.x * rhs.x.w + self.w.y * rhs.y.w + self.w.z * rhs.z.w + self.w.w * rhs.w.w,
            ),
        }
    }
}
