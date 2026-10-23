mod element;
mod export;
mod import;
mod layer;
mod render;
mod shapes;

pub(crate) use element::Element;
pub use layer::{Adjustment, Layer, LayerPosition};

use crate::Resolution;

pub struct Canvas {
    pub layers: Vec<Layer>,
    pub resolution: Resolution,
    current_layer_id: usize,
}

impl Canvas {
    pub fn new(resolution: Resolution) -> Self {
        Self {
            layers: Vec::new(),
            resolution,
            current_layer_id: 0,
        }
    }
}
