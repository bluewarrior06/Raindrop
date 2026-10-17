use crate::{
    traits::mapper::MapperCopy, types::{
        math::{color::RGBA8, vec::Vec2Du}, paint::layer::{BaseLayer, GridBasedLayer, GridLayerData, GridLayerError},
    },
};

pub struct GridLayer {
    size: Vec2Du,
    physical_size: u32,
    data: Vec<GridLayerData>,
}

impl Default for GridLayer {
    fn default() -> Self {
        Self {
            size: Vec2Du::new(1, 1),
            physical_size: 1,
            data: vec![GridLayerData::default()],
        }
    }
}

impl BaseLayer<GridLayerData> for GridLayer {}
impl MapperCopy<Vec2Du, GridLayerData, GridLayerError> for GridLayer {
    fn is_in_bounds(&self, pos: Vec2Du) -> Option<GridLayerError> {
        if pos.x >= self.size.x || pos.y >= self.size.y {
            Some(GridLayerError::SizeIsZero)
        } else {
            None
        }
    }
    fn is_size_valid(size: Vec2Du) -> Option<GridLayerError> {
        if size.x == 0 || size.y == 0 {
            Some(GridLayerError::SizeIsZero)
        } else {
            None
        }
    }

    fn get(&self, pos: Vec2Du) -> Result<GridLayerData, GridLayerError> {
        if let Some(err) = self.is_in_bounds(pos) {
            return Err(err);
        } else {
            let index = self.index_of(pos);
            return Ok(self.data[index]);
        }
    }
    fn set(&mut self, pos: Vec2Du, data: GridLayerData) -> Option<GridLayerError> {
        if let Some(err) = self.is_in_bounds(pos) {
            return Some(err);
        } else {
            let index = self.index_of(pos);
            self.data[index] = data;
            return None;
        }
    }

    fn index_of(&self, pos: Vec2Du) -> usize {
        return (pos.x + pos.y * self.size.x) as usize;
    }
    fn resize(&mut self, size: Vec2Du) -> Option<GridLayerError> {
        if let Some(err) = GridLayer::is_size_valid(size) {
            return Some(err);
        }

        self.size = size;
        let area = (self.size.x * self.size.y) as usize;
        self.data.resize(area, Default::default());

        return None;
    }
    fn get_size(&self) -> Vec2Du {
        return self.size;
    }
}

impl GridBasedLayer for GridLayer {}

impl GridLayer {
    pub fn get_physical_size(&self) -> u32 {
        self.physical_size
    }
    pub fn set_physical_size(&mut self, physical_size: u32) {
        if self.physical_size <= 0 {
            return;
        } else {
            self.physical_size = physical_size;
        }
    }
}
