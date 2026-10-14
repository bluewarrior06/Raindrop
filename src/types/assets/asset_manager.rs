use crate::types::assets::{asset_processor::AssetProcessor, asset_storage::AssetStorage};

/// Holds an asset storage and an asset processor for managing a specific type of asset.
#[derive(Default)]
pub struct AssetManager<T, P: AssetProcessor<T>> {
    storage: AssetStorage<T>,
    processor: P,
}

impl<T, P: AssetProcessor<T>> AssetManager<T, P> {
    pub fn get_storage(&self) -> &AssetStorage<T> {
        &self.storage
    }
    pub fn get_storage_mut(&mut self) -> &mut AssetStorage<T> {
        &mut self.storage
    }

    pub fn get_processor(&self) -> &P {
        &self.processor
    }
    pub fn get_processor_mut(&mut self) -> &mut P {
        &mut self.processor
    }
}
