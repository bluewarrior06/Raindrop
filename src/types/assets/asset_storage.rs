use std::collections::HashMap;

use crate::types::assets::asset_storage::AssetStorageSetError::AssetAlreadyAdded;

/// A handle on an asset.
#[derive(Hash, PartialEq, Eq)]
pub enum AssetHandle {
	/// An asset named via a string.
	Named(String)
}

/// Used to store assets. 
#[derive(Default)]
pub struct AssetStorage<T> {
	loaded: HashMap<AssetHandle, T>
}

pub enum AssetStorageSetError {
	AssetAlreadyAdded
}

impl<T> AssetStorage<T> {
	pub fn has(&self, handle: &AssetHandle) -> bool {
		self.loaded.contains_key(&handle)
	}
	pub fn set(&mut self, handle: AssetHandle, asset: Option<T>) -> Option<AssetStorageSetError> {
		if self.has(&handle) {
			return Some(AssetAlreadyAdded);
		} else {
			match asset {
				Some(asset) => self.loaded.insert(handle, asset),
				None => self.loaded.remove(&handle)
			};
			return None;
		}
		
	}
	pub fn get(&self, handle: &AssetHandle) -> Option<&T> {
		self.loaded.get(handle)
	}
	pub fn get_mut(&mut self, handle: &AssetHandle) -> Option<&mut T> {
		self.loaded.get_mut(handle)
	}
}