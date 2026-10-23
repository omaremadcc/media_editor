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
    pub(crate) id: usize,
    pub(crate) element: Element,
    pub(crate) position: LayerPosition,
    pub(crate) scale: f32,
    pub(crate) opacity: f32,
    pub(crate) adjustments_stack: Vec<Adjustment>,
}

impl Layer {
    pub fn set_x_position_end(&mut self) {
        self.position.x = vec![Position::End]
    }
    pub fn set_y_position_end(&mut self) {
        self.position.y = vec![Position::End]
    }
    pub fn set_x_position_start(&mut self) {
        self.position.x = vec![Position::Start]
    }
    pub fn set_y_position_start(&mut self) {
        self.position.y = vec![Position::Start]
    }
    pub fn center_layer(&mut self) {
        self.position = LayerPosition::center();
    }
    pub fn set_position_start(&mut self) {
        self.position = LayerPosition::start()
    }
    pub fn set_position_end(&mut self) {
        self.position = LayerPosition::end()
    }
    pub fn center_layer_x(&mut self) {
        self.position.x = vec![Position::Center];
    }
    pub fn center_layer_y(&mut self) {
        self.position.y = vec![Position::Center];
    }
    pub fn move_to_x_percentage(&mut self, percentage: f32) {
        self.position.x = vec![Position::Percent(percentage)];
    }
    pub fn move_to_percentage_y(&mut self, percentage: f32) {
        self.position.y = vec![Position::Percent(percentage)];
    }
    pub fn move_to_point_x(&mut self, x: i32) {
        self.position.x = vec![Position::Point(x)];
    }
    pub fn move_to_point_y(&mut self, y: i32) {
        self.position.y = vec![Position::Point(y)];
    }
    pub fn scale_layer(&mut self, scale: f32) {
        self.scale = scale;
    }

    pub fn move_layer_x_pixels(&mut self, pixels: i32) {
        self.position.x.push(Position::MovePixels(pixels));
    }
    pub fn move_layer_y_pixels(&mut self, pixels: i32) {
        self.position.y.push(Position::MovePixels(pixels));
    }
    pub fn move_layer_x_percent(&mut self, percentage: f32) {
        self.position.x.push(Position::MovePercent(percentage));
    }
    pub fn move_layer_y_percent(&mut self, percentage: f32) {
        self.position.y.push(Position::MovePercent(percentage));
    }
    pub fn move_layer_x_percentage_relative(&mut self, percentage: f32) {
        self.position
            .x
            .push(Position::MovePercentRelative(percentage));
    }
    pub fn move_layer_y_percentage_relative(&mut self, percentage: f32) {
        self.position
            .y
            .push(Position::MovePercentRelative(percentage));
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

    pub(crate) fn from_element(element: Element, id: usize) -> Self {
        Self {
            element,
            position: LayerPosition::start(),
            scale: 1.0,
            opacity: 1f32,
            id,
            adjustments_stack: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub struct LayerPosition {
    pub x: Vec<Position>,
    pub y: Vec<Position>,
}

impl LayerPosition {
    pub fn start() -> Self {
        return Self {
            x: vec![Position::Start],
            y: vec![Position::Start],
        };
    }
    pub fn center() -> Self {
        return Self {
            x: vec![Position::Center],
            y: vec![Position::Center],
        };
    }
    pub fn end() -> Self {
        return Self {
            x: vec![Position::End],
            y: vec![Position::End],
        };
    }
    pub fn to_translation(&self, width: f32, height: f32, resolution: &Resolution) -> (f32, f32) {
        let mut pixel_x = 0f32;
        self.x.iter().for_each(|position| match position {
            Position::Start => pixel_x = 0f32,
            Position::End => {
                if width < resolution.width as f32 {
                    pixel_x = resolution.width as f32 - width;
                } else {
                    pixel_x = 0f32;
                }
            }
            Position::Center => pixel_x = (resolution.width as f32 - width) / 2.0,
            Position::Point(point) => pixel_x = *point as f32,
            Position::Percent(percent) => pixel_x = *percent,
            Position::MovePercent(percent) => pixel_x += percent * resolution.width as f32,
            Position::MovePixels(pixels) => pixel_x += *pixels as f32,
            Position::MovePercentRelative(percent) => pixel_x += width * percent,
        });
        let mut pixel_y = 0f32;
        self.y.iter().for_each(|position| match position {
            Position::Start => pixel_y = 0f32,
            Position::End => {
                if height < resolution.height as f32 {
                    pixel_y = resolution.height as f32 - height;
                } else {
                    pixel_y = 0f32;
                }
            }
            Position::Center => pixel_y = (resolution.height as f32 - height) / 2.0,
            Position::Point(point) => pixel_y = *point as f32,
            Position::Percent(percent) => pixel_y = *percent,
            Position::MovePercent(percent) => pixel_y += percent * resolution.height as f32,
            Position::MovePixels(pixels) => pixel_y += *pixels as f32,
            Position::MovePercentRelative(percent) => pixel_y += height * percent,
        });
        return (pixel_x, pixel_y);
    }
}

#[derive(Debug)]
pub enum Position {
    Start,
    End,
    Center,
    Point(i32),
    Percent(f32),
    MovePercent(f32),
    MovePixels(i32),
    MovePercentRelative(f32),
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
