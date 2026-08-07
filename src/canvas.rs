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
    pub fn add_image(&mut self, image: Image, position: Option<crate::Point>) -> usize {
        if self.resolution.width < image.resolution.width {
            self.resolution.width = image.resolution.width;
        }
        if self.resolution.height < image.resolution.height {
            self.resolution.height = image.resolution.height;
        }
        if let Some(position) = position {
            self.add_layer(Layer { image, position });
        } else {
            self.add_layer(Layer { image, position: crate::Point { x: 0, y: 0 } });
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
            let base_index = layer.position.y * self.resolution.width as u32 + layer.position.x;
            for row in 0..layer.image.resolution.height {
                // let row = layer.image.resolution.width - row - 1;
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
    pub position: crate::Point,
}
