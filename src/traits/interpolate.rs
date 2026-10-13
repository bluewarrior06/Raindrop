pub trait Interpolate<T> {
	fn lerp(&self, other: &Self, time: T) -> Self;
}