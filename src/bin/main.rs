#![allow(unused)]
use images_editor::{Resolution, canvas::Canvas, colors::Colors};

fn main() -> () {
    let mut canvas = Canvas::new(4000, 4000);

    let rect = canvas.add_rectangle(400, 400, 0, None, Colors::red(), 0f32);
    canvas.export_to_png("output16", true);

    return ();
}
