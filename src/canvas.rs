use crate::graphic::{Graphic, GraphicType};
use crate::image::Image;
use crate::utils::scaled_pixel_color;
use crate::{Pixel, Resolution};

pub struct Canvas {
    pub layers: Vec<Layer>,
    pub resolution: Resolution,
    current_layer_id: usize,
}

impl Canvas {
    pub fn new(resolution: Resolution) -> Self {
        Self {
            layers: Vec::new(),
            resolution,
            current_layer_id: 0,
        }
    }
    fn add_layer(&mut self, layer: Layer) {
        self.layers.push(layer);
        self.current_layer_id += 1;
    }
    pub fn add_image(&mut self, image: Image, position: Option<LayerPosition>) -> usize {
        if self.resolution.width < image.resolution.width {
            self.resolution.width = image.resolution.width;
        }
        if self.resolution.height < image.resolution.height {
            self.resolution.height = image.resolution.height;
        }
        if let Some(position) = position {
            self.add_layer(Layer {
                element: Element::Image(image),
                position,
                scale: 1.0,
                id: self.current_layer_id,
            });
        } else {
            self.add_layer(Layer {
                element: Element::Image(image),
                position: LayerPosition::Default,
                scale: 1.0,
                id: self.current_layer_id,
            });
        };
        return self.layers.len() - 1;
    }

    pub fn get_mut_layer(&mut self, id: usize) -> &mut Layer {
        self.layers.iter_mut().find(|layer| layer.id == id).unwrap()
    }

    pub fn to_image(&self) -> Image {
        let num_pixels = self.resolution.width as usize * self.resolution.height as usize;
        let empty_pixels = vec![Pixel::new(255, 255, 255, None); num_pixels];
        let mut final_image = Image::new(empty_pixels, self.resolution.clone());

        let mut plot_pixel = |x: usize, y: usize, pixel: Pixel, base_index: Option<usize>| {
            if let Some(index) = base_index {
                let final_index = index + y * self.resolution.width as usize + x;
                if final_index < final_image.pixels.len() {
                    final_image.pixels[final_index] = pixel;
                }
            } else {
                let final_index = y * self.resolution.width as usize + x;
                if final_index < final_image.pixels.len() {
                    final_image.pixels[final_index] = pixel;
                }
            }
        };

        for layer in &self.layers {
            dbg!(layer.id);
            layer.element.print_type();
            match &layer.element {
                Element::Image(image) => {
                    let base_index;
                    let image = &image;
                    let mut pixels = &image.pixels;
                    let target_width = (image.resolution.width as f32 * layer.scale) as usize;
                    let target_height = (image.resolution.height as f32 * layer.scale) as usize;
                    let mut scaled_pixels =
                        vec![Pixel::new(255, 255, 255, None); target_width * target_height];

                    if layer.scale != 1.0 {
                        for row in 0..target_height {
                            for col in 0..target_width {
                                let (src_x, src_y) =
                                    scaled_pixel_color(col as u32, row as u32, 1.0 / layer.scale);
                                scaled_pixels[row * target_width + col] = image.pixels
                                    [src_y as usize * image.resolution.width + src_x as usize];
                            }
                        }
                        pixels = &scaled_pixels;
                    }
                    match layer.position {
                        LayerPosition::Default => base_index = 0,
                        LayerPosition::Point(x, y) => {
                            base_index = y * self.resolution.width as u32 + x
                        }
                        LayerPosition::Percent(x, y) => {
                            base_index = (y * self.resolution.height as f32) as u32
                                * self.resolution.width as u32
                                + (x as f32 * self.resolution.width as f32) as u32
                        }
                        LayerPosition::Center => {
                            base_index = ((self.resolution.height as f32 / 2.0)
                                - (target_height as f32 / 2.0))
                                as u32
                                * self.resolution.width as u32
                                + ((self.resolution.width as f32 / 2.0)
                                    - (target_width as f32 / 2.0))
                                    as u32
                        }
                    }
                    for row in 0..target_height {
                        for col in 0..target_width {
                            let col = target_width - col - 1;
                            let index = (row * target_width + col) as usize;
                            let pixel = pixels[index];
                            plot_pixel(col, row, pixel, Some(base_index as usize));
                        }
                    }
                }
                Element::Graphic(graphic) => match graphic.graphic_type {
                    // Basic Bresenham Algorithm
                    GraphicType::Line { start, end } => {
                        let ((x0, y0), (x1, y1)) = if start.0 > end.0 {
                            (end, start)
                        } else {
                            (start, end)
                        };

                        let stroke_color = graphic.stroke_color;

                        let dy: isize = y1 as isize - y0 as isize;
                        let dx: isize = x1 as isize - x0 as isize;

                        let width = graphic.stroke_width;

                        if dx != 0 {
                            let slope = dy / dx;
                            let mut y = y0;
                            for i in 0..(dx + 1) {
                                if width > 1 {
                                    for j in 0..width {
                                        if dy > dx {
                                            plot_pixel(
                                                x0 + i as usize + j,
                                                y,
                                                Pixel::from_hex(stroke_color),
                                                None,
                                            );
                                        } else {
                                            plot_pixel(
                                                x0 + i as usize,
                                                y + j,
                                                Pixel::from_hex(stroke_color),
                                                None,
                                            );
                                        }
                                    }
                                } else {
                                    plot_pixel(
                                        // This will work because dx is always positive (we reorder the points so x0 < x1)
                                        dbg!(x0 + i as usize),
                                        dbg!(y),
                                        Pixel::new(0, 0, 0, None),
                                        None,
                                    );
                                }

                                let py = slope * (i + 1) + y0 as isize;
                                let d0 = py - y as isize;
                                let d1 = (y as isize + 1) - py;
                                if (d0 - d1 <= 0 && slope < 0) || (d0 >= d1 && slope > 0) {
                                    if slope > 0 {
                                        y += 1;
                                        // dbg!(y);
                                    } else {
                                        y -= 1;
                                        // dbg!(y);
                                    }
                                }
                            }
                        };
                    }
                    GraphicType::Rectangle {
                        width,
                        height,
                        start,
                    } => {
                        let stroke_width = graphic.stroke_width;
                        let stroke_color = graphic.stroke_color;

                        let fill_color = graphic.fill_color;
                        let (x, y) = start;
                        for i in 0..width {
                            for j in 0..stroke_width {
                                plot_pixel(x + i, y - j, Pixel::from_hex(stroke_color), None);
                                plot_pixel(
                                    x + i,
                                    y + height + j,
                                    Pixel::from_hex(stroke_color),
                                    None,
                                );
                            }
                        }
                        // Added the stroke width to the height to fill the corners
                        for i in 0..(height + (2 * stroke_width) - 1) {
                            for j in 0..(stroke_width) {
                                plot_pixel(
                                    x - j,
                                    y + i - stroke_width + 1,
                                    Pixel::from_hex(stroke_color),
                                    None,
                                );
                                plot_pixel(
                                    x + width + j,
                                    y + i - stroke_width + 1,
                                    Pixel::from_hex(stroke_color),
                                    None,
                                );
                            }
                        }
                        for i in 1..width {
                            for j in 1..height {
                                plot_pixel(x + i, y + j, Pixel::from_hex(fill_color), None);
                            }
                        }
                    }
                },
            }
        }

        final_image
    }

    pub fn add_line(
        &mut self,
        x0: usize,
        y0: usize,
        x1: usize,
        y1: usize,
        width: usize,
        stroke_color: u32,
    ) -> usize {
        self.add_layer(Layer {
            id: self.current_layer_id,
            position: LayerPosition::Center,
            scale: 1.0,
            element: Element::Graphic(Graphic {
                graphic_type: GraphicType::Line {
                    start: (x0, y0),
                    end: (x1, y1),
                },
                stroke_width: width,
                stroke_color,
                fill_color: 0,
            }),
        });
        return self.current_layer_id - 1;
    }
    pub fn add_rectangle(
        &mut self,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
        stroke_width: usize,
        stroke_color: u32,
        fill_color: u32,
    ) -> usize {
        self.add_layer(Layer {
            id: self.current_layer_id,
            position: LayerPosition::Center,
            scale: 1.0,
            element: Element::Graphic(Graphic {
                graphic_type: GraphicType::Rectangle {
                    start: (x, y),
                    width,
                    height,
                },
                stroke_width,
                stroke_color,
                fill_color,
            }),
        });
        return self.current_layer_id - 1;
    }

    pub fn bring_layer_to_front(&mut self, layer_id: usize) {
        if let Some(index) = self.layers.iter().position(|layer| layer.id == layer_id) {
            let layer = self.layers.remove(index);
            self.layers.push(layer);
        }
    }
    pub fn bring_layer_forward(&mut self, layer_id: usize) {
        if let Some(index) = self.layers.iter().position(|layer| layer.id == layer_id) {
            let layer = self.layers.remove(index);
            self.layers.insert(index + 1, layer);
        }
    }
    pub fn send_layer_backward(&mut self, layer_id: usize) {
        println!("Hi from inside send layer backward");
        if let Some(index) = self.layers.iter().position(|layer| layer.id == layer_id) {
            let layer = self.layers.remove(index);
            self.layers.insert(index - 1, layer);
        }
    }
    pub fn send_layer_to_back(&mut self, layer_id: usize) {
        if let Some(index) = self.layers.iter().position(|layer| layer.id == layer_id) {
            let layer = self.layers.remove(index);
            self.layers.insert(0, layer);
        }
    }
}

#[derive(Debug)]
pub struct Layer {
    pub id: usize,
    pub element: Element,
    pub position: LayerPosition,
    pub scale: f32,
}

impl Layer {
    pub fn center_layer(&mut self) {
        self.position = LayerPosition::Center;
    }
    pub fn move_x_percentage(&mut self, percentage: f32) {
        if let LayerPosition::Percent(x, y) = self.position {
            self.position = LayerPosition::Percent(x + percentage, y);
        } else {
            self.position = LayerPosition::Percent(percentage, 0.0);
        }
    }
    pub fn move_y_percentage(&mut self, percentage: f32) {
        if let LayerPosition::Percent(x, y) = self.position {
            self.position = LayerPosition::Percent(x, y + percentage);
        } else {
            self.position = LayerPosition::Percent(0.0, percentage);
        }
    }
    pub fn move_to_point(&mut self, x: u32, y: u32) {
        self.position = LayerPosition::Point(x, y);
    }
    pub fn move_x(&mut self, pixels: u32) {
        if let LayerPosition::Point(x, y) = self.position {
            self.position = LayerPosition::Point(x + pixels, y);
        } else {
            self.position = LayerPosition::Point(pixels, 0);
        }
    }
    pub fn move_y(&mut self, pixels: u32) {
        if let LayerPosition::Point(x, y) = self.position {
            self.position = LayerPosition::Point(x, y + pixels);
        } else {
            self.position = LayerPosition::Point(0, pixels);
        }
    }
    pub fn scale_layer(&mut self, scale: f32) {
        self.scale = scale;
    }
}

#[derive(Debug)]
pub enum LayerPosition {
    Default,
    Point(u32, u32),
    Percent(f32, f32),
    Center,
}

#[derive(Debug)]
pub enum Element {
    Image(Image),
    Graphic(Graphic),
}

impl Element {
    pub fn is_image(&self) -> bool {
        matches!(self, Element::Image(_))
    }

    pub fn print_type(&self) {
        match self {
            Element::Image(_) => println!("Image"),
            Element::Graphic(graphic) => match graphic.graphic_type {
                GraphicType::Line { .. } => {
                    println!("Line");
                }
                GraphicType::Rectangle { .. } => {
                    println!("Rectangle");
                }
            },
        }
    }
}
