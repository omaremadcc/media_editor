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
}
