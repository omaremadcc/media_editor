use crate::Resolution;
use crate::canvas::{Canvas, Element};

impl Canvas {
    pub fn add_layer(&mut self, layer: Layer) {
        self.layers.push(layer);
        self.current_layer_id += 1;
    }

    pub fn get_mut_layer(&mut self, id: usize) -> &mut Layer {
        self.layers.iter_mut().find(|layer| layer.id == id).unwrap()
    }

    pub fn bring_layer_to_front(&mut self, layer_id: usize) {
        if let Some(index) = self.layers.iter().position(|layer| layer.id == layer_id) {
            let layer = self.layers.remove(index);
            self.layers.push(layer);
        }
    }
    pub fn bring_layer_forward(&mut self, layer_id: usize) {
        if let Some(index) = self.layers.iter().position(|layer| layer.id == layer_id) {
            let layer = self.layers.remove(index);
            self.layers.insert(index + 1, layer);
        }
    }
    pub fn send_layer_backward(&mut self, layer_id: usize) {
        // Make sure this isn't the first layer
        if self
            .layers
            .iter()
            .position(|layer| layer.id == layer_id)
            .unwrap_or(0)
            != 0
        {
            if let Some(index) = self.layers.iter().position(|layer| layer.id == layer_id) {
                let layer = self.layers.remove(index);
                self.layers.insert(index - 1, layer);
            }
        }
    }
    pub fn send_layer_to_back(&mut self, layer_id: usize) {
        if let Some(index) = self.layers.iter().position(|layer| layer.id == layer_id) {
            let layer = self.layers.remove(index);
            self.layers.insert(0, layer);
        }
    }
}

#[derive(Debug)]
pub struct Layer {
    pub id: usize,
    pub element: Element,
    pub position: LayerPosition,
    pub scale: f32,
    pub opacity: f32,
    pub adjustments_stack: Vec<Adjustment>,
}

impl Layer {
    pub fn set_x_position_end(&mut self) {
        self.position.x = Position::End
    }
    pub fn set_y_position_end(&mut self) {
        self.position.y = Position::End
    }
    pub fn set_x_position_start(&mut self) {
        self.position.x = Position::Default
    }
    pub fn set_y_position_start(&mut self) {
        self.position.y = Position::Default
    }
    pub fn center_layer(&mut self) {
        self.position = LayerPosition::center();
    }
    pub fn center_layer_x(&mut self) {
        self.position.x = Position::Center;
    }
    pub fn center_layer_y(&mut self) {
        self.position.y = Position::Center;
    }
    pub fn move_to_x_percentage(&mut self, percentage: f32) {
        self.position.x = Position::Percent(percentage);
    }
    pub fn move_to_percentage_y(&mut self, percentage: f32) {
        self.position.y = Position::Percent(percentage);
    }
    pub fn move_to_point_x(&mut self, x: u32) {
        self.position.x = Position::Point(x);
    }
    pub fn move_to_point_y(&mut self, y: u32) {
        self.position.y = Position::Point(y);
    }
    pub fn scale_layer(&mut self, scale: f32) {
        self.scale = scale;
    }

    pub fn change_exposure(&mut self, exposure: f32) {
        self.adjustments_stack.push(Adjustment::Exposure(exposure));
    }
    pub fn change_brightness(&mut self, brightness: f32) {
        self.adjustments_stack
            .push(Adjustment::Brightness(brightness));
    }
    pub fn change_saturation(&mut self, saturation: f32) {
        self.adjustments_stack
            .push(Adjustment::Saturation(saturation));
    }
    pub fn rotate_image_to_right(&mut self) {
        self.adjustments_stack.push(Adjustment::RotateRight);
    }
    pub fn rotate_image_to_left(&mut self) {
        self.adjustments_stack.push(Adjustment::RotateLeft);
    }
    pub fn mirror_image_horizontally(&mut self) {
        self.adjustments_stack.push(Adjustment::FlipHorizontal);
    }
    pub fn mirror_image_vertically(&mut self) {
        self.adjustments_stack.push(Adjustment::FlipVertical);
    }

    pub fn from_element(element: Element, id: usize) -> Self {
        Self {
            element,
            position: LayerPosition::default(),
            scale: 1.0,
            opacity: 1f32,
            id,
            adjustments_stack: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub struct LayerPosition {
    pub x: Position,
    pub y: Position,
}

impl LayerPosition {
    pub fn default() -> Self {
        return Self {
            x: Position::Default,
            y: Position::Default,
        };
    }
    pub fn center() -> Self {
        return Self {
            x: Position::Center,
            y: Position::Center,
        };
    }
    pub fn to_translation(&self, width: f32, height: f32, resolution: &Resolution) -> (f32, f32) {
        let pixel_x = match self.x {
            Position::Default => 0f32,
            Position::Point(x) => x as f32,
            Position::Percent(x) => x * resolution.width as f32,
            Position::Center => (resolution.width as f32 - width) / 2.0,
            Position::End => {
                if width < resolution.width as f32 {
                    resolution.width as f32 - width
                } else {
                    0f32
                }
            }
            Position::PointFromEnd(x) => resolution.width as f32 - width - x as f32,
        };
        let pixel_y = match self.y {
            Position::Default => 0f32,
            Position::Point(y) => y as f32,
            Position::Percent(y) => y * resolution.height as f32,
            Position::Center => (resolution.height as f32 - height) / 2.0,
            Position::End => {
                if height < resolution.height as f32 {
                    resolution.height as f32 - height
                } else {
                    0f32
                }
            }
            Position::PointFromEnd(y) => resolution.height as f32 - height - y as f32,
        };
        return (pixel_x, pixel_y);
    }
}

#[derive(Debug)]
pub enum Position {
    Default,
    Point(u32),
    PointFromEnd(u32),
    Percent(f32),
    Center,
    End,
}

#[derive(Debug)]
pub enum Adjustment {
    Exposure(f32),
    Brightness(f32),
    Saturation(f32),
    RotateRight,
    RotateLeft,
    FlipHorizontal,
    FlipVertical,
}
