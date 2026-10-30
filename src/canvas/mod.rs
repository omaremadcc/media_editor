mod element;
mod export;
mod import;
pub mod layer;
mod render;
mod shapes;

pub(crate) use element::Element;
pub use layer::{Adjustment, Layer, LayerPosition};

use crate::Resolution;

/// The main struct that holds all the layers and the resolution of the canvas.
pub struct Canvas {
    layers: Vec<Layer>,
    resolution: Resolution,
    current_layer_id: usize,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            layers: Vec::new(),
            resolution: Resolution::new(width, height),
            current_layer_id: 0,
        }
    }
    pub fn resolution(&self) -> (usize, usize) {
        return (self.resolution.width, self.resolution.height);
    }
}
