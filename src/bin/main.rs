#![allow(unused)]
use images_editor::{Resolution, canvas::Canvas};
use std::fs;

fn main() -> () {
    let mut canvas = Canvas::new(Resolution::new(1000, 1000));
    canvas.add_image_from_png("png.png");
    canvas.read_svg_from_path("bird.svg");

    canvas.to_image().write_to_png("output16.png", true);

    return ();
}
