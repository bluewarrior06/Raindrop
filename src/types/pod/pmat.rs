use bytemuck::{Pod, Zeroable};

use crate::types::pod::pvec::pVec4Df;

#[repr(C)]
#[derive(Pod, Zeroable, Debug, Default, Clone, Copy)]
pub struct pMat4x4f {
    pub x: pVec4Df,
    pub y: pVec4Df,
    pub z: pVec4Df,
    pub w: pVec4Df,
}
