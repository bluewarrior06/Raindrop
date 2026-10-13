use std::path::Path;

use crate::{traits::mapper::MapperCopy, types::{data::grad::{Grad, Grad1Df, Grad1DfEntry}, math::{color::RGBA8, vec::Vec2Du}, paint::image::{Image, RGBA8Image}}};

#[test]
pub fn test_grad_1d() {
	let mut grad: Grad1Df<RGBA8> = Grad1Df::default();
	grad.push(Grad1DfEntry::new(RGBA8::new(0, 0, 0, 255), 0.5).unwrap());
	grad.push(Grad1DfEntry::new(RGBA8::new(0, 0, 255, 255), 1.0).unwrap());
	grad.push(Grad1DfEntry::new(RGBA8::new(255, 255, 255, 255), 0.1).unwrap());
	
	let size = Vec2Du::new(100, 1);
	let mut image = RGBA8Image::new(size).unwrap();

	for x in 0..size.x {
		let t = x as f32 / size.x as f32 * 4.0;
		let intensity = grad.sample(t);

		println!("{:?}, {:?}", intensity, t);
		
		if let Some(intensity) = intensity {
			image.set(Vec2Du::new(x, 0), intensity);
		}
	}

	image.save_as_png(Path::new("image.png"));
}