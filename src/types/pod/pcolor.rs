use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Pod, Zeroable, Debug, Default, Clone, Copy)]
pub struct pRGB8 {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[repr(C)]
#[derive(Pod, Zeroable, Debug, Default, Clone, Copy)]
pub struct pRGBA8 {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}
