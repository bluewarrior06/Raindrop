/// Runs an interpolation with another Self using time T.
pub trait Interpolate<T> {
    /// Linear interpolation.
    fn lerp(&self, other: &Self, time: T) -> Self;
}
