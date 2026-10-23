use crate::Pixel;

pub(crate) fn scaled_pixel_color(x: u32, y: u32, scale: f32) -> (u32, u32) {
    let x_src = (x as f32 + 0.5) * scale - 0.5;
    let y_src = (y as f32 + 0.5) * scale - 0.5;

    return (x_src as u32, y_src as u32);
}
pub(crate) fn distance_between_point_and_line(
    point: (f32, f32),
    line_start: (f32, f32),
    line_end: (f32, f32),
) -> f32 {
    let x0 = point.0;
    let y0 = point.1;
    let x1 = line_start.0;
    let y1 = line_start.1;
    let x2 = line_end.0;
    let y2 = line_end.1;

    let numerator = ((y2 - y1) * x0 - (x2 - x1) * y0 + x2 * y1 - y2 * x1).abs();
    let denominator = ((y2 - y1).powi(2) + (x2 - x1).powi(2)).sqrt();

    numerator / denominator
}

pub(crate) fn mirror_pixels_vertically(
    pixels: &Vec<Pixel>,
    width: usize,
    height: usize,
) -> Vec<Pixel> {
    let mut new_pixels = Vec::new();

    for row in 0..height {
        let row = height - row - 1;
        for col in 0..width {
            let pixel = pixels[row * width + col];
            new_pixels.push(pixel);
        }
    }
    new_pixels
}

pub(crate) fn string_hex_to_u32(hex: &str) -> u32 {
    let hex = hex.trim_start_matches('#');
    let hex = hex.trim_start_matches("0x");
    u32::from_str_radix(hex, 16).unwrap()
}
