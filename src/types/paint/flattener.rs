use crate::types::paint::{image::Image, output::Output};

/// Flattens the results of an output onto a final image.
pub trait Flattener {
    fn flatten(&self, output: &Output, image: &mut Image);
}
