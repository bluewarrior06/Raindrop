use crate::types::paint::{
    image::{Image, RGBA8Image},
    output::Output,
};

/// Flattens the results of an output onto a final image.
pub trait Flattener {
    fn flatten(&self, output: &Output, image: &mut RGBA8Image);
}
