#![allow(unused)]
use images_editor::{
    Resolution,
    canvas::{self, Position},
    colors::Colors,
    image::Image,
    utils::calculate_little_endian,
};
use std::fs;

fn main() -> () {
    let mut canvas = canvas::Canvas::new(Resolution::new(0, 0));
    let buffer = std::fs::read("png.png").unwrap();
    let image = Image::read_from_png(&buffer).unwrap();

    canvas.add_image(image);

    let buffer = std::fs::read("low.bmp").unwrap();
    let small_image = Image::read_from_bmp(&buffer).unwrap();

    let small_image_id = canvas.add_image(small_image);

    let small_image_layer = canvas.get_mut_layer(small_image_id);
    small_image_layer.set_x_position_end();
    small_image_layer.set_y_position_end();
    small_image_layer.scale_layer(5.);
    small_image_layer.scale_layer(1.);

    let line_id = canvas.add_line(30, 500, 200, 100, 5, Colors::rose());
    let line_layer = canvas.get_mut_layer(line_id);
    line_layer.change_exposure(3.);

    let res = canvas.to_image();
    res.write_to_png("output15.png", true);
    return ();
}
