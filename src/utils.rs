pub fn calculate_little_endian(buffer: &[u8]) -> u32 {
    let mut result = 0u32;
    for (i, &byte) in buffer.iter().enumerate() {
        result |= (byte as u32) << (i * 8);
    }
    result
}

pub fn calculate_big_endian(buffer: &[u8]) -> u32 {
    let mut result = 0u32;
    for (i, &byte) in buffer.iter().enumerate() {
        result |= (byte as u32) << ((3 - i) * 8);
    }
    result
}

pub fn scaled_pixel_color(x: u32, y: u32, scale: f32) -> (u32, u32) {
    let x_src = (x as f32 + 0.5) * scale - 0.5;
    let y_src = (y as f32 + 0.5) * scale - 0.5;

    return (x_src as u32, y_src as u32);
}

pub fn quadratic_bezier(
    p0: (usize, usize),
    p1: (usize, usize),
    p2: (usize, usize),
    t: f32,
) -> (usize, usize) {
    let u = 1.0 - t;

    let point = (
        (u * u * (p0.0 as f32) + 2.0 * u * t * (p1.0 as f32) + t * t * (p2.0 as f32)),
        (u * u * (p0.1 as f32) + 2.0 * u * t * (p1.1 as f32) + t * t * (p2.1 as f32)),
    );

    return (point.0 as usize, point.1 as usize);
}
