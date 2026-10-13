use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Pod, Zeroable)]
#[derive(Debug, Default, Clone, Copy)]
pub struct pVec2Df {
	pub x: f32,
	pub y: f32
}

#[repr(C)]
#[derive(Pod, Zeroable)]
#[derive(Debug, Default, Clone, Copy)]
pub struct pVec2Di {
	pub x: i32,
	pub y: i32
}

#[repr(C)]
#[derive(Pod, Zeroable)]
#[derive(Debug, Default, Clone, Copy)]
pub struct pVec3Df {
	pub x: f32,
	pub y: f32,
	pub z: f32
}

#[repr(C)]
#[derive(Pod, Zeroable)]
#[derive(Debug, Default, Clone, Copy)]
pub struct pVec3Di {
	pub x: i32,
	pub y: i32,
	pub z: i32
}

#[repr(C)]
#[derive(Pod, Zeroable)]
#[derive(Debug, Default, Clone, Copy)]
pub struct pVec4Df {
	pub x: f32,
	pub y: f32,
	pub z: f32,
	pub w: f32
}

#[repr(C)]
#[derive(Pod, Zeroable)]
#[derive(Debug, Default, Clone, Copy)]
pub struct pVec4Di {
	pub x: i32,
	pub y: i32,
	pub z: i32,
	pub w: i32
}