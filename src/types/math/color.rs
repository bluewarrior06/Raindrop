use crate::traits::interpolate::Interpolate;

pub type Red8 = Red<u8>;

/// Contains a single red color channel.
/// Data type defined through T.
#[derive(Debug, Default, Clone, Copy)]
pub struct Red<T> {
    pub r: T,
}

pub type RGB8 = RGB<u8>;

/// Contains color information in channels RGB.
/// Data type defined through T.
#[derive(Debug, Default, Clone, Copy)]
pub struct RGB<T> {
    pub r: T,
    pub g: T,
    pub b: T,
}

pub type RGBA8 = RGBA<u8>;

#[repr(C)]
/// Contains color information in channels RGBA.
/// Data type defined through T.
#[derive(Debug, Default, Clone, Copy)]
pub struct RGBA<T> {
    pub r: T,
    pub g: T,
    pub b: T,
    pub a: T,
}

impl RGBA8 {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }
}

impl Interpolate<f32> for RGBA8 {
    fn lerp(&self, other: &Self, time: f32) -> Self {
        let r = (self.r as f32 + time * (other.r as f32 - self.r as f32)).floor() as u8;
        let g = (self.g as f32 + time * (other.g as f32 - self.g as f32)).floor() as u8;
        let b = (self.b as f32 + time * (other.b as f32 - self.b as f32)).floor() as u8;
        let a = (self.a as f32 + time * (other.a as f32 - self.a as f32)).floor() as u8;

        RGBA8 { r, g, b, a }
    }
}
