use crate::canvas::{Canvas, Element, Layer};
use crate::formats::svg::read_svg_from_path;
use crate::image::Image;
use std::io::Error;

impl Canvas {
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
        return self.current_layer_id;
    }

    pub fn read_svg_from_path(&mut self, path: &str) -> usize {
        let svg_data = read_svg_from_path(path).unwrap();
        let element = Element::SVG(svg_data);
        self.add_layer(Layer::from_element(element, self.current_layer_id));
        return self.current_layer_id - 1;
    }

    pub fn add_image_from_png(&mut self, path: &str) -> Result<usize, Error> {
        let image = Image::read_from_png_path(path)?;
        let element = Element::Image(image);
        self.add_layer(Layer::from_element(element, self.current_layer_id));
        return Ok(self.current_layer_id - 1);
    }

    pub fn add_image_from_bmp(&mut self, path: &str) -> Result<usize, Error> {
        let image = Image::read_from_bmp_path(path)?;
        let element = Element::Image(image);
        self.add_layer(Layer::from_element(element, self.current_layer_id));
        return Ok(self.current_layer_id - 1);
    }
}
