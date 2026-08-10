use crate::utils::scaled_pixel_color;
use crate::{Pixel, Resolution};
use crate::image::Image;

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
    fn add_layer(&mut self, layer: Layer) {
        self.layers.push(layer);
        self.current_layer_id += 1;
    }
    pub fn add_image(&mut self, image: Image, position: Option<LayerPosition>) -> usize {
        if self.resolution.width < image.resolution.width {
            self.resolution.width = image.resolution.width;
        }
        if self.resolution.height < image.resolution.height {
            self.resolution.height = image.resolution.height;
        }
        if let Some(position) = position {
            self.add_layer(Layer { image, position, scale: 1.0, id: self.current_layer_id });
        } else {
            self.add_layer(Layer { image, position: LayerPosition::Default, scale: 1.0, id: self.current_layer_id });
        };
        return self.layers.len() - 1;
    }

    pub fn get_mut_layer(&mut self, id: usize) -> &mut Layer {
        self.layers.iter_mut().find(|layer| layer.id == id).unwrap()
    }


    pub fn to_image(&self) -> Image {
        let num_pixels = self.resolution.width as usize * self.resolution.height as usize;
        let empty_pixels = vec![Pixel::new(255, 255, 255, None); num_pixels];
        let mut image = Image::new(empty_pixels, self.resolution.clone());

        for layer in &self.layers {
            let base_index;
            let mut pixels = &layer.image.pixels;
            let target_width = (layer.image.resolution.width as f32 * layer.scale) as usize;
            let target_height = (layer.image.resolution.height as f32 * layer.scale) as usize;
            let mut scaled_pixels = vec![Pixel::new(255, 255, 255, None); target_width * target_height];

            if layer.scale != 1.0 {

                for row in 0..target_height {
                    for col in 0..target_width {
                        let (src_x, src_y) = scaled_pixel_color(col as u32, row as u32, 1.0 / layer.scale);
                        scaled_pixels[row * target_width + col] = layer.image.pixels[src_y as usize * layer.image.resolution.width + src_x as usize];
                    }
                }
                pixels = &scaled_pixels;
            }
            match layer.position {
                LayerPosition::Default => base_index = 0,
                LayerPosition::Point(x, y) => base_index = y * self.resolution.width as u32 + x,
                LayerPosition::Percent(x, y) => base_index = (y * self.resolution.height as f32) as u32 * self.resolution.width as u32 + (x as f32 * self.resolution.width as f32) as u32,
                LayerPosition::Center => base_index = ((self.resolution.height as f32 / 2.0) - (target_height as f32 / 2.0)) as u32 * self.resolution.width as u32 + ((self.resolution.width as f32 / 2.0) - (target_width as f32 / 2.0)) as u32,
            }
            for row in 0..target_height {
                for col in 0..target_width {
                    let col = target_width - col - 1;
                    let index = (row * target_width + col) as usize;
                    let pixel = pixels[index];
                    let index = (row * image.resolution.width + col) as usize;
                    let index_final = base_index as usize + index;
                    if index_final < image.pixels.len() {
                        image.pixels[index_final] = pixel;
                    }
                }
            }
        }

        image
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
        if let Some(index) = self.layers.iter().position(|layer| layer.id == layer_id) {
            let layer = self.layers.remove(index);
            self.layers.insert(index - 1, layer);
        }
    }
    pub fn send_layer_to_back(&mut self, layer_id: usize) {
        if let Some(index) = self.layers.iter().position(|layer| layer.id == layer_id) {
            let layer = self.layers.remove(index);
            self.layers.insert(0, layer);
        }
    }
}

pub struct Layer {
    pub id: usize,
    pub image: Image,
    pub position: LayerPosition,
    pub scale: f32,
}

impl Layer {
    pub fn center_layer(&mut self) {
        self.position = LayerPosition::Center;
    }
    pub fn move_x_percentage(&mut self, percentage: f32) {
        if let LayerPosition::Percent(x, y) = self.position {
            self.position = LayerPosition::Percent(x + percentage, y);
        } else {
            self.position = LayerPosition::Percent(percentage, 0.0);
        }
    }
    pub fn move_y_percentage(&mut self, percentage: f32) {
        if let LayerPosition::Percent(x, y) = self.position {
            self.position = LayerPosition::Percent(x, y + percentage);
        } else {
            self.position = LayerPosition::Percent(0.0, percentage);
        }
    }
    pub fn move_to_point(&mut self, x: u32, y: u32) {
        self.position = LayerPosition::Point(x, y);
    }
    pub fn move_x(&mut self, pixels: u32) {
        if let LayerPosition::Point(x, y) = self.position {
            self.position = LayerPosition::Point(x + pixels, y);
        } else {
            self.position = LayerPosition::Point(pixels, 0);
        }
    }
    pub fn move_y(&mut self, pixels: u32) {
        if let LayerPosition::Point(x, y) = self.position {
            self.position = LayerPosition::Point(x, y + pixels);
        } else {
            self.position = LayerPosition::Point(0, pixels);
        }
    }
    pub fn scale_layer(&mut self, scale: f32) {
        self.scale = scale;
    }
}


pub enum LayerPosition {
    Default,
    Point(u32, u32),
    Percent(f32, f32),
    Center
}
