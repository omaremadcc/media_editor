use crate::image::Image;
use crate::utils;
use crate::{Pixel, Resolution};
use flate2::Compression;
use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;
use std::io::{Error, Read, Write};

impl Image {
    pub fn read_from_png(buffer: &[u8]) -> Result<Self, Error> {
        let signature = &buffer[..8];
        if signature != &[137, 80, 78, 71, 13, 10, 26, 10] {
            return Err(Error::new(
                std::io::ErrorKind::InvalidData,
                "invalid PNG signature",
            ));
        }

        let width = utils::calculate_big_endian(&buffer[16..20]);
        let height = utils::calculate_big_endian(&buffer[20..24]);
        let bit_depth = buffer[24];
        let color_type = buffer[25];
        let number_of_channels = match color_type {
            0 => 1,
            2 => 3,
            3 => 1,
            4 => 2,
            6 => 4,
            _ => {
                return Err(Error::new(
                    std::io::ErrorKind::InvalidData,
                    "invalid color type",
                ));
            }
        };

        let mut idat_concat = Vec::new();

        let mut index = 33;
        loop {
            let chunk_length = utils::calculate_big_endian(&buffer[index..index + 4]);
            let chunk_type = &buffer[(index + 4)..(index + 8)];

            if chunk_type == b"IEND" {
                break;
            }

            if chunk_type == b"IDAT" {
                let chunk_data = &buffer[(index + 8)..(index + 8 + chunk_length as usize)];
                idat_concat.extend_from_slice(chunk_data);
            }

            // Skip chunk data and CRC
            index += 8 + chunk_length as usize + 4;
        }

        let mut decoder = ZlibDecoder::new(&idat_concat[..]);
        let mut uncompressed_data = Vec::new();
        decoder.read_to_end(&mut uncompressed_data).unwrap();

        let bytes_per_pixel = (number_of_channels * bit_depth as usize) / 8; // RGBA8, for example
        let stride = 1 + width as usize * bytes_per_pixel;

        let mut decoded_data = Vec::new();

        for y in 0..height as usize {
            let row_start = y * stride as usize;

            let filter = uncompressed_data[row_start];
            let filtered = &uncompressed_data[row_start + 1..row_start + stride as usize];

            let mut row = vec![0u8; filtered.len()];

            for x in 0..filtered.len() {
                let raw = filtered[x];

                let left = if x >= bytes_per_pixel as usize {
                    row[x - bytes_per_pixel as usize]
                } else {
                    0
                };

                let above = if y > 0 {
                    decoded_data[(y - 1) * filtered.len() + x]
                } else {
                    0
                };

                let above_left = if y > 0 && x >= bytes_per_pixel as usize {
                    decoded_data[(y - 1) * filtered.len() + x - bytes_per_pixel as usize]
                } else {
                    0
                };

                row[x] = match filter {
                    0 => raw,

                    1 => raw.wrapping_add(left),

                    2 => raw.wrapping_add(above),

                    3 => {
                        let average = ((left as u16 + above as u16) / 2) as u8;
                        raw.wrapping_add(average)
                    }

                    4 => {
                        let p = left as i32 + above as i32 - above_left as i32;

                        let pa = (p - left as i32).abs();
                        let pb = (p - above as i32).abs();
                        let pc = (p - above_left as i32).abs();

                        let predictor = if pa <= pb && pa <= pc {
                            left
                        } else if pb <= pc {
                            above
                        } else {
                            above_left
                        };

                        raw.wrapping_add(predictor)
                    }

                    _ => panic!("invalid PNG filter: {filter}"),
                };
            }

            decoded_data.extend_from_slice(&row);
        }

        let pixels = if bytes_per_pixel == 3 {
            decoded_data
                .chunks_exact(3)
                .map(|c| Pixel::new(c[0], c[1], c[2], 255))
                .collect::<Vec<_>>()
        } else {
            decoded_data
                .chunks_exact(4)
                .map(|c| Pixel::new(c[0], c[1], c[2], c[3]))
                .collect::<Vec<_>>()
        };

        let mut image = Image::new(pixels, Resolution::new(width as usize, height as usize));

        // Reverse the pixel because we use bottom to top arrangement
        image.mirror_image_vertically();

        Ok(image)
    }

    pub fn write_to_png(&self, path: &str, transparent: bool) -> Result<(), Error> {
        let mut file = Vec::new();

        // Add PNG signature
        file.extend_from_slice(&[0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A]);

        // Add IHDR chunk
        let mut ihdr = Vec::with_capacity(13);
        println!("start: {}", ihdr.len());

        ihdr.extend_from_slice(&(self.resolution.width as u32).to_be_bytes());
        println!("after width: {}", ihdr.len());

        ihdr.extend_from_slice(&(self.resolution.height as u32).to_be_bytes());
        println!("after height: {}", ihdr.len());

        ihdr.extend_from_slice(&[8, 6, 0, 0, 0]);
        println!("after metadata: {}", ihdr.len());
        append_chunk(&mut file, b"IHDR", &ihdr);

        println!("{}", ihdr.len());

        // Raw scanlines, using filter type 0 (None).
        let width = self.resolution.width as usize;
        let height = self.resolution.height as usize;

        let mut raw = Vec::with_capacity(height * (1 + width * 4));

        for y in 0..height {
            raw.push(0); // Filter type: None

            for x in 0..width {
                let p = &self.pixels[y * width + x];
                raw.extend_from_slice(&[p.r, p.g, p.b, p.a]);
            }
        }

        // Compress all scanlines into one zlib stream.
        let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(&raw)?;
        let compressed = encoder.finish()?;

        append_chunk(&mut file, b"IDAT", &compressed);
        append_chunk(&mut file, b"IEND", &[]);

        std::fs::write(path, file)?;

        // println!(
        //     "width: {}, height: {}, pixels: {}",
        //     self.resolution.width,
        //     self.resolution.height,
        //     self.pixels.len()
        // );

        Ok(())
    }
}

fn append_chunk(file: &mut Vec<u8>, kind: &[u8; 4], data: &[u8]) {
    file.extend_from_slice(&(data.len() as u32).to_be_bytes());
    file.extend_from_slice(kind);
    file.extend_from_slice(data);

    let mut crc_data = Vec::with_capacity(4 + data.len());
    crc_data.extend_from_slice(kind);
    crc_data.extend_from_slice(data);

    let mut hasher = crc32fast::Hasher::new();
    hasher.update(&crc_data);
    file.extend_from_slice(&hasher.finalize().to_be_bytes());
}
