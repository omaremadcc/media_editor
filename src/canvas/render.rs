use crate::canvas::{Canvas, Element};
use crate::formats::svg::SVGData;
use crate::image::Image;
use crate::utils::scaled_pixel_color;
use crate::{Pixel, Resolution};

impl Canvas {
    pub(crate) fn to_image(&self) -> Image {
        let num_pixels = self.resolution.width as usize * self.resolution.height as usize;
        let empty_pixels = vec![Pixel::empty(); num_pixels];
        let mut final_image = Image::new(empty_pixels, self.resolution.clone());

        fn plot_pixel(
            x: usize,
            y: usize,
            pixel: Pixel,
            base_index: Option<usize>,
            final_image: &mut Image,
            canvas_resolution: &Resolution,
        ) {
            if y > canvas_resolution.height as usize || x > canvas_resolution.width as usize {
                return;
            }
            let index = base_index.unwrap_or(0) + y * canvas_resolution.width as usize + x;
            // Make sure the pixel is not out of bounds
            if index < final_image.pixels.len()
                && x <= canvas_resolution.width
                && y <= canvas_resolution.height
            {
                if pixel.a != 255 && !final_image.pixels[index].is_empty() {
                    final_image.pixels[index] = final_image.pixels[index].merge_with(&pixel);
                    // final_image.pixels[index] = pixel;
                } else {
                    final_image.pixels[index] = pixel;
                }
            }
        }

        // THIS FUNCTION WAS WRITTEN BY AI
        fn straight_line_from_to(
            (x0, y0): (usize, usize),
            (x1, y1): (usize, usize),
            width: usize,
            stroke_color: u32,
            final_image: &mut Image,
            canvas_resolution: &Resolution,
        ) {
            if width == 0 {
                return;
            }

            let (mut x0, mut y0) = (x0 as isize, y0 as isize);
            let (x1, y1) = (x1 as isize, y1 as isize);

            let dx = (x1 - x0).abs();
            let dy = (y1 - y0).abs();

            let sx = if x0 < x1 { 1 } else { -1 };
            let sy = if y0 < y1 { 1 } else { -1 };

            let mut error = dx - dy;

            // Use a disk of radius width / 2 to create a continuous stroke.
            let radius = width as f64 / 2.0;
            let radius_squared = radius * radius;

            loop {
                let min_x = (x0 - radius.ceil() as isize).max(0);
                let min_y = (y0 - radius.ceil() as isize).max(0);
                let max_x = x0 + radius.ceil() as isize;
                let max_y = y0 + radius.ceil() as isize;

                for py in min_y..=max_y {
                    for px in min_x..=max_x {
                        let ddx = px as f64 - x0 as f64;
                        let ddy = py as f64 - y0 as f64;

                        if ddx * ddx + ddy * ddy <= radius_squared {
                            plot_pixel(
                                px as usize,
                                py as usize,
                                Pixel::from_hex(stroke_color),
                                None,
                                final_image,
                                canvas_resolution,
                            );
                        }
                    }
                }

                if x0 == x1 && y0 == y1 {
                    break;
                }

                let e2 = 2 * error;

                if e2 > -dy {
                    error -= dy;
                    x0 += sx;
                }

                if e2 < dx {
                    error += dx;
                    y0 += sy;
                }
            }
        }

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

                    let (pixel_x, pixel_y) = layer.position.to_translation(
                        target_width as f32,
                        target_height as f32,
                        &self.resolution,
                    );

                    let base_index = pixel_x + (pixel_y * self.resolution.width as f32);
                    for row in 0..target_height {
                        for col in 0..target_width {
                            let col = target_width - col - 1;
                            let index = (row * target_width + col) as usize;

                            let mut pixel = pixels[index];
                            pixel.a = (pixel.a as f32 * layer.opacity) as u8;
                            plot_pixel(
                                col,
                                row,
                                pixel,
                                Some(base_index as usize),
                                &mut final_image,
                                &self.resolution,
                            );
                        }
                    }
                }
                Element::Graphic(graphic) => graphic.graphic_type.render(
                    graphic.stroke_width,
                    graphic.stroke_color,
                    graphic.fill_color,
                    &layer.position,
                    layer.scale,
                    &mut final_image,
                    &self.resolution,
                    straight_line_from_to,
                    plot_pixel,
                ),
                Element::SVG(SVGData { elements, .. }) => {
                    for graphic in elements {
                        graphic.graphic_type.render(
                            graphic.stroke_width,
                            graphic.stroke_color,
                            graphic.fill_color,
                            &layer.position,
                            layer.scale,
                            &mut final_image,
                            &self.resolution,
                            straight_line_from_to,
                            plot_pixel,
                        );
                    }
                }
            }
        }

        final_image
    }
}
