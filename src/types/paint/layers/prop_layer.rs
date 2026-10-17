use crate::types::paint::layer::{BaseLayer, PropBasedLayer, PropLayerData, PropLayerError};

pub struct PropLayer {
    props: Vec<PropLayerData>,
}

impl BaseLayer<PropLayerData> for PropLayer {}

impl PropBasedLayer for PropLayer {
    fn has_at(&self, at: usize) -> bool {
        at < self.props.len()
    }

    fn push(&mut self, data: PropLayerData) {
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

    fn push_at(&mut self, data: PropLayerData, index: usize) -> Option<PropLayerError> {
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

    fn get(&mut self, index: usize) -> Result<PropLayerData, PropLayerError> {
        match self.props.get(index) {
            Some(prop) => Ok(prop.clone()),
            None => Err(PropLayerError::IndexOutOfBounds),
        }
    }
}
