use std::{collections::HashMap, hash::Hash, marker::PhantomData};

use iced::widget::sensor::Key;

use crate::types::associations::instantiator::Instantiator;

pub struct Associator<T, Err, K: Eq + Hash, I: Instantiator<T, Err>> {
	associations: HashMap<K, I>,

	_a: PhantomData<T>,
	_b: PhantomData<Err>
}

impl<T, Err, K: Eq + Hash, I: Instantiator<T, Err>> Associator<T, Err, K, I> {
	pub fn associate(&mut self, key: K, instantiator: I) {
		self.associations.insert(key, instantiator);
	}
	pub fn disassociate(&mut self, key: &K) -> Option<I> {
		self.associations.remove(key)
	}
	pub fn has_association(&self, key: &K) -> bool {
		self.associations.contains_key(&key)
	}

	pub fn instantiate(&mut self, key: &K) -> Option<Result<T, Err>> {
		match self.associations.get(key) {
			Some(instantiator) => Some(instantiator.instantiate()),
			None => None
		}
	}
}
