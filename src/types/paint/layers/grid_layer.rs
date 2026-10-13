use iced::window::position;

use crate::{
    traits::mapper::MapperCopy,
    types::{
        math::{color::RGBA8, vec::Vec2Du},
        paint::layer::{BaseLayer, GridBasedLayer, GridLayerError},
    },
};

pub type IndexedGridLayer = GridLayer<usize>;
pub type ColoredGridLayer = GridLayer<RGBA8>;

pub struct GridLayer<T>
where
    T: Default + Clone + Copy,
{
    size: Vec2Du,
    data: Vec<T>,
}

impl<T> Default for GridLayer<T>
where
    T: Default + Clone + Copy,
{
    fn default() -> Self {
        Self {
            size: Vec2Du::new(1, 1),
            data: vec![T::default()],
        }
    }
}

impl<T> BaseLayer<T> for GridLayer<T> where T: Default + Clone + Copy {}
impl<T> MapperCopy<Vec2Du, T, GridLayerError> for GridLayer<T>
where
    T: Default + Clone + Copy,
{
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

    fn get(&self, pos: Vec2Du) -> Result<T, GridLayerError> {
        if let Some(err) = self.is_in_bounds(pos) {
            return Err(err);
        } else {
            let index = self.index_of(pos);
            return Ok(self.data[index]);
        }
    }
    fn set(&mut self, pos: Vec2Du, data: T) -> Option<GridLayerError> {
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
        if let Some(err) = GridLayer::<T>::is_size_valid(size) {
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

impl<T> GridBasedLayer<T> for GridLayer<T> where T: Default + Clone + Copy {}
