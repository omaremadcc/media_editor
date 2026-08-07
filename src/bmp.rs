use std::io::Read;

use super::{Pixel, Resolution, utils::calculate_little_endian};

#[derive(Debug)]
pub struct Bmp {
    pub file_size: u32,
    pub resolution: Resolution,
    pub pixels: Vec<Pixel>,
}

impl Bmp {
    pub fn new(file_size: u32, resolution: Resolution, pixels: Vec<Pixel>) -> Self {
        Self {
            file_size,
            resolution,
            pixels,
        }
    }

    pub fn read_from_file(file_path: &str) -> Result<Self, std::io::Error> {
        let file = std::fs::File::open(file_path)?;
        let mut reader = std::io::BufReader::new(file);
        let mut buffer = Vec::new();
        reader.read_to_end(&mut buffer)?;

        let file_size = calculate_little_endian(&buffer[2..6]);
        let pixel_offset = calculate_little_endian(&buffer[10..14]) as usize;
        let width = calculate_little_endian(&buffer[18..22]);
        let height = calculate_little_endian(&buffer[22..26]);
        let image_data_size = calculate_little_endian(&buffer[34..38]) as usize;

        let row_ideal_size = ((3 * width + 3) / 4) * 4;
        let row_padding = (row_ideal_size - 3 * width) as usize;
        println!("row_ideal_size: {row_ideal_size}, row_padding: {row_padding}");

        let pixels = Vec::new();
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

        for _chunk in no_padding_pixel_bytes.chunks_exact(3) {
            // pixels.push(Pixel::new(chunk[2], chunk[1], chunk[0]));
        }

        Ok(Self::new(
            file_size as u32,
            Resolution::new(width as usize, height as usize),
            pixels,
        ))
    }

    pub fn write_to_file(self, path: &str) -> Result<(), std::io::Error> {
        let mut file = Vec::new();
        // Magic numbers for bmp format
        file.push(0x42);
        file.push(0x4D);
        // file size
        for byte in (self.file_size as u32).to_le_bytes() {
            file.push(byte);
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
        file.push(24);
        file.push(0);
        // compression
        for _ in 0..4 {
            file.push(0);
        }
        // image data size
        let width = self.resolution.width;
        let row_ideal_size = (width * 3 + 3) & !3;
        let row_padding = row_ideal_size - 3 * width;
        for byte in ((row_ideal_size * self.resolution.height) as u32).to_le_bytes() {
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
                file.extend_from_slice(&pixel.to_bgr());
            }
            // 2. Append padding at the end of the row
            file.extend(std::iter::repeat(0).take(row_padding as usize));
        }
        // Write to file
        std::fs::write(path, file).map_err(|e| e)
    }
}
