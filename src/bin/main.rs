#![allow(unused)]
use images_editor::{
    Resolution,
    canvas::{self, Element, Layer, Position},
    colors::Colors,
    graphic::{
        Graphic, GraphicType,
        PathCommand::{self, CubicTo, LineTo, MoveTo, QuadraticTo},
    },
    image::Image,
    svg::{parse_svg_path, svg_to_path},
    utils::calculate_little_endian,
};
use std::fs;

fn main() -> () {
    let mut canvas = canvas::Canvas::new(Resolution::new(0, 0));
    let buffer = std::fs::read("png.png").unwrap();
    let image = Image::read_from_png(&buffer).unwrap();

    canvas.add_image(image);

    let svg_path = String::from(
        "
 M3 2a1 1 0 0 0-1 1v10a1 1 0 0 0 1 1h14a1 1 0 0 0 1-1V3a1 1 0 0 0-1-1H3zm0-2h14a3 3 0 0 1 3 3v10a3 3 0 0 1-3 3H3a3 3 0 0 1-3-3V3a3 3 0 0 1 3-3z       ",
    );
    let layer_id = canvas.add_svg_path(&svg_path, 1, Colors::white(), Colors::yellow());

    let res = canvas.to_image();
    res.write_to_png("output15.png", true);
    return ();
}
