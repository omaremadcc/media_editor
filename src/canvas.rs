use crate::{Pixel, Resolution};
use crate::image::Image;

pub struct Canvas {
    pub layers: Vec<Layer>,
    pub resolution: Resolution,
}

impl Canvas {
    pub fn new(resolution: Resolution) -> Self {
        Self {
            layers: Vec::new(),
            resolution,
        }
    }
    fn add_layer(&mut self, layer: Layer) {
        self.layers.push(layer);
    }
    pub fn add_image(&mut self, image: Image, position: Option<LayerPosition>) -> usize {
        if self.resolution.width < image.resolution.width {
            self.resolution.width = image.resolution.width;
        }
        if self.resolution.height < image.resolution.height {
            self.resolution.height = image.resolution.height;
        }
        if let Some(position) = position {
            self.add_layer(Layer { image, position });
        } else {
            self.add_layer(Layer { image, position: LayerPosition::Default });
        };
        return self.layers.len() - 1;
    }

    pub fn get_mut_layer(&mut self, index: usize) -> &mut Layer {
        &mut self.layers[index]
    }


    pub fn to_image(&self) -> Image {
        let num_pixels = self.resolution.width as usize * self.resolution.height as usize;
        let empty_pixels = vec![Pixel::new(255, 255, 255, None); num_pixels];
        let mut image = Image::new(empty_pixels, self.resolution.clone());

        for layer in &self.layers {
            let base_index;
            match layer.position {
                LayerPosition::Default => base_index = 0,
                LayerPosition::Point(x, y) => base_index = y * self.resolution.width as u32 + x,
                LayerPosition::Percent(x, y) => base_index = (y * self.resolution.height as f32) as u32 * self.resolution.width as u32 + (x as f32 * self.resolution.width as f32) as u32,
                LayerPosition::Center => base_index = ((self.resolution.height as f32 / 2.0) - (layer.image.resolution.height as f32 / 2.0)) as u32 * self.resolution.width as u32 + ((self.resolution.width as f32 / 2.0) - (layer.image.resolution.width as f32 / 2.0)) as u32,
            }
            for row in 0..layer.image.resolution.height {
                for col in 0..layer.image.resolution.width {
                    let col = layer.image.resolution.width - col - 1;
                    let index = (row * layer.image.resolution.width + col) as usize;
                    let pixel = layer.image.pixels[index];
                    let index = (row * image.resolution.width + col) as usize;
                    image.pixels[base_index as usize + index] = pixel;
                }
            }
        }

        image
    }
}

pub struct Layer {
    pub image: Image,
    pub position: LayerPosition,
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

}


pub enum LayerPosition {
    Default,
    Point(u32, u32),
    Percent(f32, f32),
    Center
}
