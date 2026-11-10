#![allow(unused)]
use images_editor::{Resolution, canvas::Canvas, colors::Colors};

fn main() -> () {
    let mut canvas = Canvas::new(2000, 2000);
    canvas.add_rectangle(2000, 2000, 0, None, Colors::white(), 0.);
    let id = canvas.add_circle(100., 10, Colors::green(), Colors::blue());
    canvas.get_mut_layer(id).center_layer();

    let id = canvas.read_svg_from_path("bird(6).svg");
    canvas.get_mut_layer(id).scale_layer(5.);
    // canvas.get_mut_layer(id).set_x_position_end();
    // canvas.get_mut_layer(id).move_layer_y_percent(0.3);

    canvas.export_to_png("output18.png", false);
    return ();
}
