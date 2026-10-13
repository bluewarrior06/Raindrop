pub type RGBA8 = RGBA<u8>;

/// Contains a single red color channel.
/// Data type defined through T.
#[derive(Debug, Default, Clone, Copy)]
pub struct Red<T> {
    pub r: T,
}

/// Contains color information in channels RGB.
/// Data type defined through T.
#[derive(Debug, Default, Clone, Copy)]
pub struct RGB<T> {
    pub r: T,
    pub g: T,
    pub b: T,
}

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
