use crate::image::Image;
use crate::utils;
use crate::{Pixel, Resolution};
use flate2::read::ZlibDecoder;
use std::io::{Error, Read};

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
}
