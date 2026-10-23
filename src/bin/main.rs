#![allow(unused)]
use images_editor::{Resolution, canvas::Canvas};

fn main() -> () {
    let mut canvas = Canvas::new(1000, 1000);
    canvas.add_image_from_png("png.png");
    canvas.read_svg_from_path("bird.svg");

    return ();
}
