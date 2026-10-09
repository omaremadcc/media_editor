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
    pub fn add_image(&mut self, image: Image) -> usize {
        if self.resolution.width < image.resolution.width {
            self.resolution.width = image.resolution.width;
        }
        if self.resolution.height < image.resolution.height {
            self.resolution.height = image.resolution.height;
        }
        self.add_layer(Layer::from_element(
            Element::Image(image),
            self.current_layer_id,
        ));
        return self.layers.len() - 1;
    }

    pub fn get_mut_layer(&mut self, id: usize) -> &mut Layer {
        self.layers.iter_mut().find(|layer| layer.id == id).unwrap()
    }

    pub fn to_image(&self) -> Image {
        let num_pixels = self.resolution.width as usize * self.resolution.height as usize;
        let empty_pixels = vec![Pixel::empty(); num_pixels];
        let mut final_image = Image::new(empty_pixels, self.resolution.clone());

        let mut plot_pixel = |x: usize, y: usize, pixel: Pixel, base_index: Option<usize>| {
            let index = base_index.unwrap_or(0) + y * self.resolution.width as usize + x;
            if index < final_image.pixels.len() {
                if pixel.a != 255 && !final_image.pixels[index].is_empty() {
                    final_image.pixels[index] = final_image.pixels[index].merge_with(&pixel);
                    // final_image.pixels[index] = pixel;
                } else {
                    final_image.pixels[index] = pixel;
                }
            }
        };

        for layer in &self.layers {
            match &layer.element {
                Element::Image(image) => {
                    // Adjust image
                    let mut image = image.clone();
                    image.apply_adjustments(&layer.adjustments_stack);

                    let image = &image;
                    let mut pixels = &image.pixels;
                    let target_width = (image.resolution.width as f32 * layer.scale) as usize;
                    let target_height = (image.resolution.height as f32 * layer.scale) as usize;
                    let mut scaled_pixels = vec![Pixel::empty(); target_width * target_height];

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
                    let pixel_x = match layer.position.x {
                        Position::Default => 0,
                        Position::Point(x) => x,
                        Position::Percent(x) => (x as f32 * self.resolution.width as f32) as u32,
                        Position::Center => {
                            ((self.resolution.width as f32 / 2.0) - (target_width as f32 / 2.0))
                                as u32
                        }
                        Position::End => (self.resolution.width - target_width) as u32,
                    };
                    let pixel_y = match layer.position.y {
                        Position::Default => 0,
                        Position::Point(y) => y,
                        Position::Percent(y) => (y * self.resolution.height as f32) as u32,
                        Position::Center => {
                            ((self.resolution.height as f32 / 2.0) - (target_height as f32 / 2.0))
                                as u32
                        }
                        Position::End => (self.resolution.height - target_height) as u32,
                    };
                    let base_index = pixel_x + (pixel_y * self.resolution.width as u32);
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
                                        Pixel::empty(),
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
        let element = Element::Graphic(Graphic {
            graphic_type: GraphicType::Line {
                start: (x0, y0),
                end: (x1, y1),
            },
            stroke_width: width,
            stroke_color,
            fill_color: 0,
        });
        self.add_layer(Layer::from_element(element, self.current_layer_id));
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
        let element = Element::Graphic(Graphic {
            graphic_type: GraphicType::Rectangle {
                start: (x, y),
                width,
                height,
            },
            stroke_width,
            stroke_color,
            fill_color,
        });
        self.add_layer(Layer::from_element(element, self.current_layer_id));
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
    pub adjustments_stack: Vec<Adjustment>,
}

impl Layer {
    pub fn set_x_position_end(&mut self) {
        self.position.x = Position::End
    }
    pub fn set_y_position_end(&mut self) {
        self.position.y = Position::End
    }
    pub fn set_x_position_start(&mut self) {
        self.position.x = Position::Default
    }
    pub fn set_y_position_start(&mut self) {
        self.position.y = Position::Default
    }
    pub fn center_layer(&mut self) {
        self.position = LayerPosition::center();
    }
    pub fn center_layer_x(&mut self) {
        self.position.x = Position::Center;
    }
    pub fn center_layer_y(&mut self) {
        self.position.y = Position::Center;
    }
    pub fn move_x_percentage(&mut self, percentage: f32) {
        self.position.x = Position::Percent(percentage);
    }
    pub fn move_to_percentage_y(&mut self, percentage: f32) {
        self.position.y = Position::Percent(percentage);
    }
    pub fn move_to_point_x(&mut self, x: u32) {
        self.position.x = Position::Point(x);
    }
    pub fn move_to_point_y(&mut self, y: u32) {
        self.position.y = Position::Point(y);
    }
    pub fn scale_layer(&mut self, scale: f32) {
        self.scale = scale;
    }

    pub fn change_exposure(&mut self, exposure: f32) {
        self.adjustments_stack.push(Adjustment::Exposure(exposure));
    }
    pub fn change_brightness(&mut self, brightness: f32) {
        self.adjustments_stack
            .push(Adjustment::Brightness(brightness));
    }
    pub fn change_saturation(&mut self, saturation: f32) {
        self.adjustments_stack
            .push(Adjustment::Saturation(saturation));
    }
    pub fn rotate_image_to_right(&mut self) {
        self.adjustments_stack.push(Adjustment::RotateRight);
    }
    pub fn rotate_image_to_left(&mut self) {
        self.adjustments_stack.push(Adjustment::RotateLeft);
    }
    pub fn mirror_image_horizontally(&mut self) {
        self.adjustments_stack.push(Adjustment::FlipHorizontal);
    }
    pub fn mirror_image_vertically(&mut self) {
        self.adjustments_stack.push(Adjustment::FlipVertical);
    }

    pub fn from_element(element: Element, id: usize) -> Self {
        Self {
            element,
            position: LayerPosition::default(),
            scale: 1.0,
            id,
            adjustments_stack: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub struct LayerPosition {
    pub x: Position,
    pub y: Position,
}

impl LayerPosition {
    pub fn default() -> Self {
        return Self {
            x: Position::Default,
            y: Position::Default,
        };
    }
    pub fn center() -> Self {
        return Self {
            x: Position::Center,
            y: Position::Center,
        };
    }
}

#[derive(Debug)]
pub enum Position {
    Default,
    Point(u32),
    Percent(f32),
    Center,
    End,
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

#[derive(Debug)]
pub enum Adjustment {
    Exposure(f32),
    Brightness(f32),
    Saturation(f32),
    RotateRight,
    RotateLeft,
    FlipHorizontal,
    FlipVertical,
}
