use crate::{Resolution, utils::calculate_little_endian};
use std::io::Error;

use super::Pixel;

pub struct Image {
    pub pixels: Vec<Pixel>,
    pub resolution: Resolution,
}

impl Image {
    pub fn new(pixels: Vec<Pixel>, resolution: Resolution) -> Self {
        Self { pixels, resolution }
    }

    pub fn read_from_bmp(buffer: &[u8]) -> Result<Self, Error> {
        let pixel_offset = calculate_little_endian(&buffer[10..14]) as usize;
        let width = calculate_little_endian(&buffer[18..22]) as usize;
        let height = calculate_little_endian(&buffer[22..26]);
        let bits_per_pixel = calculate_little_endian(&buffer[28..30]) as usize;

        // Total row width in bytes (padded to a 4-byte / 32-bit boundary)
        let row_padded_bytes = ((bits_per_pixel * width + 31) / 32) * 4;

        // Unpadded row length in bytes (rounded up to nearest byte for sub-byte bpp)
        // let row_unpadded_bytes = (bits_per_pixel * width + 7) / 8;

        // Padding required at the end of each row, in bytes
        // let row_padding = (row_padded_bytes - row_unpadded_bytes) as usize;

        let mut pixels = Vec::new();
        let mut no_padding_pixel_bytes = Vec::new();

        for (index, byte) in buffer[pixel_offset..]
            .into_iter()
            .enumerate()
        {
            let mod_index = index % (row_padded_bytes) as usize;
            if (mod_index as usize) >= (width as usize * 3) {
                continue;
            }
            no_padding_pixel_bytes.push(byte.clone());
        }
        for chunk in no_padding_pixel_bytes.chunks_exact(bits_per_pixel / 8) {
            let alpha = if bits_per_pixel == 32 {
                Some(chunk[3])
            } else {
                None
            };
            pixels.push(Pixel::new(chunk[2], chunk[1], chunk[0], alpha));
        }

        Ok(Self {
            pixels,
            resolution: Resolution {
                width: width as usize,
                height: height as usize,
            },
        })
    }

    pub fn write_to_bmp(&self, path: &str) -> Result<(), Error> {
        let mut file = Vec::new();
        // Magic numbers for bmp format
        file.push(0x42);
        file.push(0x4D);
        // File Size, empty for now
        for _ in 0..4 {
            file.push(0);
        }
        // Unused bytes
        for _ in 0..4 {
            file.push(0);
        }
        // Pixel Data offset placeholder
        for _ in 0..4 {
            file.push(0);
        }
        // Header Size
        file.push(40);
        for _ in 0..3 {
            file.push(0);
        }
        // Image Width
        let image_width = self.resolution.width;
        for byte in (image_width as u32).to_le_bytes() {
            file.push(byte);
        }
        // Image Height
        for byte in (self.resolution.height as u32).to_le_bytes() {
            file.push(byte);
        }
        // Color planes
        file.push(1);
        file.push(0);
        // Bits per pixel
        file.push(24 as u8);
        file.push(0);
        // compression
        for _ in 0..4 {
            file.push(0);
        }
        // image data size
        let bytes_per_pixel = 3 as usize;
        let width = self.resolution.width;
        let row_ideal_size = (width * bytes_per_pixel + bytes_per_pixel) & !bytes_per_pixel;
        let row_padding = row_ideal_size - bytes_per_pixel * width;
        for byte in (row_ideal_size as u32 * self.resolution.height as u32).to_le_bytes() {
            file.push(byte);
        }
        // unused
        for _ in 0..16 {
            file.push(0);
        }

        println!("row_ideal_size: {row_ideal_size}, row_padding: {row_padding}");

        // Adding pixel offset
        for i in 0..4 {
            file[10 + i] = (file.len() as u32).to_le_bytes()[i];
        }

        for row in self.pixels.chunks_exact(width as usize) {
            // 1. Write pixel bytes for this row
            for pixel in row {
                file.extend_from_slice(&pixel.to_bgr());
            }
            // 2. Append padding at the end of the row
            file.extend(std::iter::repeat(0).take(row_padding as usize));
        }

        // Update file size
        let file_size = file.len() as u32;
        for (index, byte) in file_size.to_le_bytes().iter().enumerate() {
            file[2 + index] = byte.clone();
        }

        // Write to file
        std::fs::write(path, file).map_err(|e| e)
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
    pub fn change_hue(&mut self, factor: f32) {
        self.pixels.iter_mut().for_each(|pixel| {
            pixel.shift_hue(factor);
        });
    }

    pub fn change_saturation(&mut self, factor: f32) {
        self.pixels.iter_mut().for_each(|pixel| {
            pixel.change_saturation(factor);
        });
    }


    pub fn rotate_image_to_right(&mut self) {
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

    pub fn rotate_image_to_left(&mut self) {
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

    pub fn mirror_image_vertically(&mut self) {
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

    pub fn mirror_image_horizontally(&mut self) {
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

}
