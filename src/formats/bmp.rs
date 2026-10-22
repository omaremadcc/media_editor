use crate::image::Image;
use crate::utils::mirror_pixels_vertically;
use crate::{Pixel, Resolution};
use std::io::Error;

impl Image {
    pub fn read_from_bmp(buffer: &[u8]) -> Result<Self, Error> {
        let pixel_offset = u32::from_le_bytes(buffer[10..14].try_into().unwrap()) as usize;
        let width = u32::from_le_bytes(buffer[18..22].try_into().unwrap()) as usize;
        let height = u32::from_le_bytes(buffer[22..26].try_into().unwrap()) as usize;
        let bits_per_pixel = u16::from_le_bytes(buffer[28..30].try_into().unwrap()) as usize;

        // Total row width in bytes (padded to a 4-byte / 32-bit boundary)
        let row_padded_bytes = ((bits_per_pixel * width + 31) / 32) * 4;

        // Unpadded row length in bytes (rounded up to nearest byte for sub-byte bpp)
        // let row_unpadded_bytes = (bits_per_pixel * width + 7) / 8;

        // Padding required at the end of each row, in bytes
        // let row_padding = (row_padded_bytes - row_unpadded_bytes) as usize;

        let mut pixels = Vec::new();
        let mut no_padding_pixel_bytes = Vec::new();

        for (index, byte) in buffer[pixel_offset..].into_iter().enumerate() {
            let mod_index = index % (row_padded_bytes) as usize;
            let width_per_pixel = if bits_per_pixel == 32 { 4 } else { 3 };
            if (mod_index as usize) >= (width as usize * width_per_pixel) {
                continue;
            }
            no_padding_pixel_bytes.push(byte.clone());
        }

        for chunk in no_padding_pixel_bytes.chunks_exact(bits_per_pixel / 8) {
            let alpha = if bits_per_pixel == 32 { chunk[3] } else { 255 };

            pixels.push(Pixel::new(chunk[2], chunk[1], chunk[0], alpha));
        }

        let pixels = mirror_pixels_vertically(&pixels, width, height);

        Ok(Self {
            pixels,
            resolution: Resolution {
                width: width as usize,
                height: height as usize,
            },
        })
    }

    pub fn write_to_bmp(&self, path: &str, is_alpha: bool) -> Result<(), Error> {
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
        if is_alpha {
            file.push(108);
        } else {
            file.push(40);
        }
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
        if is_alpha {
            file.push(32 as u8);
        } else {
            file.push(24 as u8);
        }
        file.push(0);
        // compression
        if is_alpha {
            file.push(3);
        } else {
            file.push(0);
        }
        for _ in 0..3 {
            file.push(0);
        }
        // image data size
        let bytes_per_pixel = if is_alpha { 4 } else { 3 } as usize;
        let width = self.resolution.width;
        let height = self.resolution.height;
        let row_ideal_size = (width * bytes_per_pixel + bytes_per_pixel) & !bytes_per_pixel;
        let row_padding = row_ideal_size - bytes_per_pixel * width;
        for byte in (row_ideal_size as u32 * self.resolution.height as u32).to_le_bytes() {
            file.push(byte);
        }
        // unused
        for _ in 0..16 {
            file.push(0);
        }

        // This is the new header info v5
        // RGBA explicit masks for alpha channel
        if is_alpha {
            // Red Mask
            file.extend_from_slice(&[0, 0, 0xFF, 0]);
            // Green Mask
            file.extend_from_slice(&[0, 0xFF, 0, 0]);
            // Blue Mask
            file.extend_from_slice(&[0xFF, 0, 0, 0]);
            // Alpha Mask
            file.extend_from_slice(&[0, 0, 0, 0xFF]);
            for _ in 0..52 {
                file.push(0);
            }
        }

        // Adding pixel offset
        for i in 0..4 {
            file[10 + i] = (file.len() as u32).to_le_bytes()[i];
        }

        let pixels = mirror_pixels_vertically(&self.pixels, width, height);

        for row in pixels.chunks_exact(width as usize) {
            // 1. Write pixel bytes for this row
            for pixel in row {
                if is_alpha {
                    // file.extend_from_slice(&pixel.to_bgra());
                    file.extend_from_slice(&pixel.to_bgra())
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

        let mut path = String::from(path);
        if !path.ends_with(".bmp") {
            path.push_str(".bmp");
        }

        // Write to file
        std::fs::write(path, file).map_err(|e| e)
    }
}
