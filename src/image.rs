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
        let file_size = calculate_little_endian(&buffer[2..6]);
        let pixel_offset = calculate_little_endian(&buffer[10..14]) as usize;
        let width = calculate_little_endian(&buffer[18..22]);
        let height = calculate_little_endian(&buffer[22..26]);
        let image_data_size = calculate_little_endian(&buffer[34..38]) as usize;
        let bits_per_pixel = calculate_little_endian(&buffer[28..30]) as usize;

        let row_ideal_size = ((3 * width + 3) / 4) * 4;
        let row_padding = (row_ideal_size - 3 * width) as usize;
        println!("row_ideal_size: {row_ideal_size}, row_padding: {row_padding}");

        let mut pixels = Vec::new();
        let mut no_padding_pixel_bytes = Vec::new();

        for (index, byte) in buffer[pixel_offset..(image_data_size + pixel_offset)]
            .into_iter()
            .enumerate()
        {
            let mod_index = index % (row_ideal_size) as usize;
            if (mod_index as usize) >= (width as usize * 3) {
                continue;
            }
            no_padding_pixel_bytes.push(byte.clone());
        }
        println!(
            "no_padding_pixel_bytes_length: {:?}",
            no_padding_pixel_bytes.len()
        );
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
        let is_alpha = self.pixels.iter().any(|p| p.a.is_some());
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
        // Pixel Data offset
        file.push(54);
        for _ in 0..3 {
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
        file.push(24 + is_alpha as u8 * 8);
        file.push(0);
        // compression
        for _ in 0..4 {
            file.push(0);
        }
        // image data size
        let bytes_per_pixel = 3 + is_alpha as usize;
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

        for row in self.pixels.chunks_exact(width as usize) {
            // 1. Write pixel bytes for this row
            for pixel in row {
                if is_alpha {
                    file.extend_from_slice(&pixel.to_bgra());
                } else {
                    file.extend_from_slice(&pixel.to_bgr());
                }
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
}
