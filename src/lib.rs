pub mod bmp;
pub mod image;
pub mod utils;

#[derive(Debug)]
pub struct Resolution {
    pub width: usize,
    pub height: usize,
}
impl Resolution {
    pub fn new(width: usize, height: usize) -> Self {
        Self { width, height }
    }
}

#[derive(Debug)]
pub struct Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: Option<u8>,
}
impl Pixel {
    pub fn new(r: u8, g: u8, b: u8, a: Option<u8>) -> Self {
        Self { r, g, b, a }
    }

    pub fn to_bgr(&self) -> [u8; 3] {
        [self.b, self.g, self.r]
    }

    pub fn to_bgra(&self) -> [u8; 4] {
        [self.b, self.g, self.r, self.a.unwrap_or(0)]
    }

    pub fn to_hex(&self) -> u32 {
        ((self.b as u32) << 16) | ((self.g as u32) << 8) | (self.r as u32)
    }

    pub fn normalize(&self) -> f32 {
        ((self.r as f32) / 255.0 + (self.g as f32) / 255.0 + (self.b as f32) / 255.0) / 3.0
    }

    pub fn change_exposure(&mut self, factor: f32) {
        self.r = adjust_pixel_channel_exposure(self.r, factor);
        self.g = adjust_pixel_channel_exposure(self.g, factor);
        self.b = adjust_pixel_channel_exposure(self.b, factor);
    }

    pub fn change_brightness(&mut self, factor: f32) {
        self.r = adjust_pixel_channel_brightness(self.r, factor);
        self.g = adjust_pixel_channel_brightness(self.g, factor);
        self.b = adjust_pixel_channel_brightness(self.b, factor);
    }
}

fn adjust_pixel_channel_exposure(channel: u8, factor: f32) -> u8 {
    let mut linear = (channel as f32 / 255.0).powf(2.2);

    let base = 2.0f32;

    linear *= base.powf(factor);

    let srgb = linear.powf(1.0 / 2.2) * 255.0;

    if srgb == 0.0 {return 0};
    if srgb == 255.0 {return 255};

    return srgb as u8;
}

fn adjust_pixel_channel_brightness(channel: u8, factor: f32) -> u8 {
    let linear = (channel as f32 / 255.0).powf(2.2);

    let srgb = (linear + factor * (1.0 - (linear - 0.5).abs() * 2.0) ) * 255.0;

    if srgb == 0.0 {return 0};
    if srgb == 255.0 {return 255};

    return srgb as u8;
}
