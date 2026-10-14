use crate::{traits::interpolate::Interpolate, types::math::color::RGBA8};

/// An individual entry inside of a gradient.
pub trait DeltaGradEntry<T, M>: Clone + Copy
where
    T: Interpolate<M> + Clone + Copy,
{
    /// Gets the delta of the gradient.
    /// The delta is the position relative to the last entry in a gradient.
    fn get_delta(&self) -> M;
    /// Sets the delta of the current gradient entry.
    /// Has to always be (delta > zero).
    fn set_delta(&mut self, pos: M);

    /// Sets the value of the entry.
    fn get_value(&self) -> T;
    /// Gets the value of the entry.
    fn set_value(&mut self, val: T);

    /// Interpolates the entry with another entry.
    fn interpolate(&self, with: &Self, time: M) -> T;
}

/// A gradient stores "entries" at positions to then return an interpolated value.
/// For all implementations of Grad, entries are designed to be sorted.
/// To easily keep thing always sorted, elemen deltas can only be a positive value (delta > zero).
pub trait DeltaGrad<T, M, E>
where
    E: DeltaGradEntry<T, M>,
    T: Interpolate<M> + Clone + Copy,
{
    /// Pushes a new gradient entry.
    fn push(&mut self, entry: E);
    /// Pops the last added entry.
    fn pop(&mut self);

    /// Checks to see if an element exists at a given index.
    fn has_element_at(&self, index: usize) -> bool;

    /// Pushes a new gradient entry at a specific index.
    fn push_at(&mut self, entry: E, index: usize);
    /// Pops a gradient entry at a specific index.
    fn pop_at(&mut self, index: usize) -> Option<E>;

    /// Returns (Total delta prior to left, left, right)
    fn find_pair(&self, at: M) -> (M, Option<&E>, Option<&E>);

    /// Gets a gradient entry at a specific entry.
    fn get_at(&self, at: usize) -> Option<&E>;

    /// Gets the upper bound of the gradient.
    /// All gradients have a lower bounds of zero (or what ever the lowest quantity that T can be).
    fn get_upper_bound(&self) -> M;

    /// Samples the gradient.
    fn sample(&self, at: M) -> Option<T>;
}

#[derive(Clone, Copy)]
pub struct DeltaGrad1DfEntry<T>
where
    T: Interpolate<f32> + Clone + Copy,
{
    value: T,
    delta: f32,
}

impl<T> DeltaGrad1DfEntry<T>
where
    T: Interpolate<f32> + Clone + Copy,
{
    pub fn new(value: T, delta: f32) -> Option<DeltaGrad1DfEntry<T>> {
        if delta <= 0.0 {
            None
        } else {
            Some(Self {
                value: value,
                delta: delta,
            })
        }
    }
}

impl<T> DeltaGradEntry<T, f32> for DeltaGrad1DfEntry<T>
where
    T: Interpolate<f32> + Clone + Copy,
{
    fn get_delta(&self) -> f32 {
        self.delta
    }
    fn set_delta(&mut self, pos: f32) {
        self.delta = pos.max(f32::MIN);
    }

    fn get_value(&self) -> T {
        return self.value;
    }
    fn set_value(&mut self, val: T) {
        self.value = val;
    }

    fn interpolate(&self, with: &DeltaGrad1DfEntry<T>, time: f32) -> T {
        return self.value.lerp(&with.value, time);
    }
}

#[derive(Default)]
pub struct DeltaGrad1Df<T>
where
    T: Interpolate<f32> + Clone + Copy,
{
    entries: Vec<DeltaGrad1DfEntry<T>>,
}

impl<T> DeltaGrad<T, f32, DeltaGrad1DfEntry<T>> for DeltaGrad1Df<T>
where
    T: Interpolate<f32> + Clone + Copy,
{
    fn push(&mut self, entry: DeltaGrad1DfEntry<T>) {
        self.entries.push(entry);
    }

    fn pop(&mut self) {
        self.entries.pop();
    }

    fn has_element_at(&self, index: usize) -> bool {
        index < self.entries.len()
    }
    fn push_at(&mut self, entry: DeltaGrad1DfEntry<T>, index: usize) {
        if self.has_element_at(index) {
            self.entries.insert(index, entry);
        } else {
            return;
        }
    }
    fn pop_at(&mut self, index: usize) -> Option<DeltaGrad1DfEntry<T>> {
        if self.has_element_at(index) {
            return Some(self.entries.remove(index));
        } else {
            return None;
        }
    }

    fn find_pair(&self, at: f32) -> (f32, Option<&DeltaGrad1DfEntry<T>>, Option<&DeltaGrad1DfEntry<T>>) {
        if self.entries.len() == 0 {
            return (0.0, None, None);
        }

        let mut left: Option<&DeltaGrad1DfEntry<T>> = None;
        let mut right: Option<&DeltaGrad1DfEntry<T>> = None;

        let mut delta = 0.0;
        let mut total_delta_before_left: f32 = 0.0;

        for (index, element) in self.entries.iter().enumerate() {
            delta += element.delta;

            if delta >= at {
                total_delta_before_left = delta - element.delta;
                left = Some(element);
                right = self.entries.get(index + 1);
                break;
            }
        }

        return (total_delta_before_left, left, right);
    }

    fn get_at(&self, at: usize) -> Option<&DeltaGrad1DfEntry<T>> {
        self.entries.get(at)
    }

    fn get_upper_bound(&self) -> f32 {
        let mut bound = 0.0;

        for e in self.entries.iter() {
            bound += e.delta;
        }

        return bound;
    }

    fn sample(&self, at: f32) -> Option<T> {
        if self.entries.len() == 0 {
            return None;
        }

        let (total_delta_before_left, left, right) = self.find_pair(at);

        match (left, right) {
            (Some(left), Some(right)) => {
                return Some(left.interpolate(right, (at - total_delta_before_left) / left.delta));
            }
            (_, _) => return None,
        };
    }
}
