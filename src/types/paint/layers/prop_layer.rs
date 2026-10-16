use crate::types::paint::layer::{BaseLayer, PropBasedError, PropLayerError};

pub struct PropLayer<T: Default + Clone + Copy> {
	props: Vec<T>
}

impl<T: Default + Clone + Copy> BaseLayer<T> for PropLayer<T> {}

impl<T: Default + Clone + Copy> PropBasedError<T> for PropLayer<T> {
	fn has_at(&self, at: usize) -> bool {
		at < self.props.len()	
	}
	
	fn push(&mut self, data: T) {
		self.props.push(data);
	}
	fn pop(&mut self) -> Option<PropLayerError> {
		if self.props.len() == 0 {
			Some(PropLayerError::NothingToPop)
		} else {
			self.props.pop();
			None
		}
	}

	fn push_at(&mut self, data: T, index: usize) -> Option<PropLayerError> {
		if !self.has_at(index) {
			Some(PropLayerError::IndexOutOfBounds)
		} else {
			self.props.insert(index, data);
			None
		}
	}
	fn pop_at(&mut self, index: usize) -> Option<PropLayerError> {
		if !self.has_at(index) {
			Some(PropLayerError::IndexOutOfBounds)
		} else {
			self.props.remove(index);
			None
		}
	}

	fn get(&mut self, index: usize) -> Result<T, PropLayerError> {
		match self.props.get(index) {
			Some(prop) => Ok(prop.clone()),
			None => Err(PropLayerError::IndexOutOfBounds)
		}
	}
}