use crate::{
    traits::mapper::MapperCopy,
    types::paint::{
        flattener::Flattener,
        image::{Image, RGBA8Image},
        output::Output,
    },
};

/// Used to "flatten" the images inside of an output onto a final image.
#[derive(Default)]
pub struct Flattened {
    flatteners: Vec<Box<dyn Flattener>>,
}

impl Flattened {
    pub fn add_flattener(&mut self, flattener: Box<dyn Flattener>) {
        self.flatteners.push(flattener);
    }

    pub fn flatten(&mut self, output: &Output) -> RGBA8Image {
        let albedo = output.get_albedo();
        let mut image = Image::default();
        image.resize(albedo.get_size());

        for flattener in self.flatteners.iter_mut() {
            flattener.flatten(output, &mut image);
        }

        return image;
    }
}
