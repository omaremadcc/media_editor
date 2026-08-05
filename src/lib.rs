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

    pub fn to_hsv(&self) -> (f32, f32, f32) {
        let r = self.r as f32 / 255.0;
        let g = self.g as f32 / 255.0;
        let b = self.b as f32 / 255.0;

        let max = r.max(g).max(b);
        let min = r.min(g).min(b);
        let delta = max - min;
        let v = max;
        let s = if max == 0.0 { 0.0 } else { delta / max };
        let mut h;

        if delta == 0.0 {
            return (0.0, s, v);
        }

        if r == max {
            h = (g - b) / delta;
        } else if g == max {
            h = 2.0 + (b - r) / delta;
        } else {
            h = 4.0 + (r - g) / delta;
        }

        h *= 60.0;
        if h < 0.0 {
            h += 360.0;
        }

        (h, s, v)
    }

    pub fn shift_hue(&mut self, degree: f32) {
        let hsv = self.to_hsv();
        let mut h = (hsv.0 + degree) % 360.0;
        if h < 0.0 {
            h += 360.0;
        }

        let (h, s, v) = (h, hsv.1, hsv.2);

        let (r, g, b) = hsv_to_rgb(h, s, v);
        self.r = r;
        self.g = g;
        self.b = b;
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

fn hsv_to_rgb(h: f32, s: f32, v: f32) -> (u8, u8, u8) {
    let chroma = v * s;
    let h_degree = h / 60.0;
    let x = chroma * (1.0 - ((h_degree % 2.0 - 1.0).abs()));
    let m = (v - chroma) as f32;

    let res;

    if 0.0 <= h_degree && h_degree < 1.0 {
        res = (chroma, x, 0.0);
    } else if 1.0 <= h_degree && h_degree < 2.0 {
        res = (x, chroma, 0.0);
    } else if 2.0 <= h_degree && h_degree < 3.0 {
        res = (0.0, chroma, x);
    } else if 3.0 <= h_degree && h_degree < 4.0 {
        res = (0.0, x ,chroma);
    } else if 4.0 <= h_degree && h_degree < 5.0 {
        res = (x, 0.0, chroma);
    } else if 5.0 <= h_degree && h_degree < 6.0 {
        res = (chroma, 0.0, x);
    } else {
        res = (0.0, 0.0, 0.0);
    };

    let r = ((res.0 + m) * 255.0).round().clamp(0.0, 255.0) as u8;
    let g = ((res.1 + m) * 255.0).round().clamp(0.0, 255.0) as u8;
    let b = ((res.2 + m) * 255.0).round().clamp(0.0, 255.0) as u8;

    return (r, g, b);
}
