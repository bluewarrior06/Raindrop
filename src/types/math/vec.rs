use crate::types::pod::pvec::{pVec2Df, pVec3Df, pVec4Df};

// Vector 2
pub type Vec2Df = Vec2D<f32>;
pub type Vec2Di = Vec2D<i32>;
pub type Vec2Du = Vec2D<u32>;

#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct Vec2D<T> {
	pub x: T,
	pub y: T
}

impl Into<pVec2Df> for Vec2Df {
	fn into(self) -> pVec2Df {
		pVec2Df {
			x: self.x,
			y: self.y
		}
	}
}

impl<T> Vec2D<T> {
	pub fn new(x: T, y: T) -> Self {
		Self{x, y}
	}
}

// Vector 3
pub type Vec3Df = Vec3D<f32>;
pub type Vec3Di = Vec3D<i32>;
pub type Vec3Du = Vec3D<u32>;

#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct Vec3D<T> {
	pub x: T,
	pub y: T,
	pub z: T
}

impl Into<pVec3Df> for Vec3Df {
	fn into(self) -> pVec3Df {
		pVec3Df {
			x: self.x,
			y: self.y,
			z: self.z
		}
	}
}

impl<T> Vec3D<T> {
	pub fn new(x: T, y: T, z: T) -> Self {
		Self {x, y, z}
	}
}

// Vector 4
pub type Vec4Df = Vec4D<f32>;
pub type Vec4Di = Vec4D<i32>;
pub type Vec4Du = Vec4D<u32>;

#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct Vec4D<T> {
	pub x: T,
	pub y: T,
	pub z: T,
	pub w: T
}

impl Into<pVec4Df> for Vec4Df {
	fn into(self) -> pVec4Df {
		pVec4Df {
			x: self.x,
			y: self.y,
			z: self.z,
			w: self.w
		}
	}
}

impl<T> Vec4D<T> {
	pub fn new(x: T, y: T, z: T, w: T) -> Self {
		Self {x, y, z, w}
	}
}