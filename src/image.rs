use super::Pixel;
use crate::Resolution;
use crate::canvas::Adjustment;

#[derive(Debug, Clone)]
pub(crate) struct Image {
    pub(crate) pixels: Vec<Pixel>,
    pub(crate) resolution: Resolution,
}

impl Image {
    pub fn new(pixels: Vec<Pixel>, resolution: Resolution) -> Self {
        Self { pixels, resolution }
    }

    pub fn change_exposure(&mut self, factor: f32) {
        self.pixels.iter_mut().for_each(|pixel| {
            pixel.change_exposure(factor);
        });
    }
    pub fn change_brightness(&mut self, factor: f32) {
        self.pixels.iter_mut().for_each(|pixel| {
            pixel.change_brightness(factor);
        });
    }
    pub(crate) fn _change_hue(&mut self, factor: f32) {
        self.pixels.iter_mut().for_each(|pixel| {
            pixel._shift_hue(factor);
        });
    }

    pub(crate) fn change_saturation(&mut self, factor: f32) {
        self.pixels.iter_mut().for_each(|pixel| {
            pixel.change_saturation(factor);
        });
    }

    pub(crate) fn rotate_image_to_right(&mut self) {
        let height = self.resolution.height;
        let width = self.resolution.width;
        let mut new_pixels = Vec::new();

        for col in 0..width {
            let col = width - col - 1;
            for row in 0..height {
                let pixel = self.pixels[row * width + col];
                new_pixels.push(pixel);
            }
        }
        self.resolution.width = height;
        self.resolution.height = width;
        self.pixels = new_pixels;
    }

    pub(crate) fn rotate_image_to_left(&mut self) {
        let height = self.resolution.height;
        let width = self.resolution.width;
        let mut new_pixels = Vec::new();

        for col in 0..width {
            for row in 0..height {
                let row = height - row - 1;
                let pixel = self.pixels[row * width + col];
                new_pixels.push(pixel);
            }
        }
        self.resolution.width = height;
        self.resolution.height = width;
        self.pixels = new_pixels;
    }

    pub(crate) fn mirror_image_vertically(&mut self) {
        let mut new_pixels = Vec::new();

        for row in 0..self.resolution.height {
            let row = self.resolution.height - row - 1;
            for col in 0..self.resolution.width {
                let pixel = self.pixels[row * self.resolution.width + col];
                new_pixels.push(pixel);
            }
        }
        self.pixels = new_pixels;
    }

    pub(crate) fn mirror_image_horizontally(&mut self) {
        let mut new_pixels = Vec::new();

        for row in 0..self.resolution.height {
            for col in 0..self.resolution.width {
                let col = self.resolution.width - col - 1;
                let pixel = self.pixels[row * self.resolution.width + col];
                new_pixels.push(pixel);
            }
        }
        self.pixels = new_pixels;
    }

    pub(crate) fn apply_adjustments(&mut self, adjustments: &Vec<Adjustment>) {
        for adjustment in adjustments {
            match adjustment {
                Adjustment::Exposure(exposure) => {
                    self.change_exposure(*exposure);
                }
                Adjustment::Brightness(brightness) => {
                    self.change_brightness(*brightness);
                }
                Adjustment::Saturation(saturation) => {
                    self.change_saturation(*saturation);
                }
                Adjustment::RotateRight => {
                    self.rotate_image_to_right();
                }
                Adjustment::RotateLeft => {
                    self.rotate_image_to_left();
                }
                Adjustment::FlipHorizontal => {
                    self.mirror_image_horizontally();
                }
                Adjustment::FlipVertical => {
                    self.mirror_image_vertically();
                }
            }
        }
    }
}
