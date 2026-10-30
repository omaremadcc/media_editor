use crate::Pixel;

pub(crate) fn scaled_pixel_color(x: u32, y: u32, scale: f32) -> (u32, u32) {
    let x_src = (x as f32 + 0.5) * scale - 0.5;
    let y_src = (y as f32 + 0.5) * scale - 0.5;

    return (x_src as u32, y_src as u32);
}
pub(crate) fn distance_between_point_and_ray(
    point: (f32, f32),
    ray_start: (f32, f32),
    ray_end: (f32, f32),
) -> f32 {
    let (x0, y0) = point;
    let (x1, y1) = ray_start;
    let (x2, y2) = ray_end;

    let vx = x2 - x1;
    let vy = y2 - y1;
    let v_len_sq = vx * vx + vy * vy;

    // Handle degenerate ray (start and end are the same point)
    if v_len_sq == 0.0 {
        let dx = x0 - x1;
        let dy = y0 - y1;
        return (dx * dx + dy * dy).sqrt();
    }

    let apx = x0 - x1;
    let apy = y0 - y1;

    // Compute projection parameter t using dot product
    let dot = apx * vx + apy * vy;
    let t = dot / v_len_sq;

    if t < 0.0 {
        // Behind the ray origin -> distance to ray_start
        (apx * apx + apy * apy).sqrt()
    } else {
        // On or ahead of the ray origin -> perpendicular distance to the infinite line
        let numerator = ((y2 - y1) * x0 - (x2 - x1) * y0 + x2 * y1 - y2 * x1).abs();
        let denominator = v_len_sq.sqrt();
        numerator / denominator
    }
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
