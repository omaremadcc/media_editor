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
        let pixel_offset = calculate_little_endian(&buffer[10..14]);
        let width = calculate_little_endian(&buffer[18..22]);
        let height = calculate_little_endian(&buffer[22..26]);

        let mut pixels = Vec::new();
        for pixel_data in buffer[pixel_offset as usize..].chunks(3) {
            pixels.push(Pixel::new(pixel_data[2], pixel_data[1], pixel_data[0])); // RGB is stored as BGR in BMP files so we reverse it
        }

        Ok(Self::new(file_size as u32, Resolution::new(width, height), pixels))
    }
}
