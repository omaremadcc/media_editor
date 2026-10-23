#![allow(unused)]
use images_editor::{
    Resolution,
    canvas::{Canvas, Layer, Position},
    colors::Colors,
    image::Image,
};
use std::fs;

fn main() -> () {
    //     let mut canvas = Canvas::new(Resolution::new(0, 0));
    //     let buffer = std::fs::read("png.png").unwrap();
    //     let image = Image::read_from_png(&buffer).unwrap();
    //
    //     canvas.add_image(image);
    //
    //
    //     let text_layer_id = canvas.add_text(
    //         String::from("omar\nemad"),
    //         50,
    //         1,
    //         Some(Colors::green()),
    //         None,
    //     );
    //     let layer = canvas.get_mut_layer(text_layer_id);
    //     layer.scale = 2.0;
    //
    //     let text_layer = canvas.get_mut_layer(text_layer_id);
    //     text_layer.position.y = Position::PointFromEnd(100);
    //
    //     let circle_layer_id = canvas.add_circle(100., 1, Some(Colors::red()), Some(Colors::blue()));
    //
    //     let id = canvas.add_rectangle(200, 200, 1, Some(Colors::red()), None);
    //     let layer = canvas.get_mut_layer(id);
    //     layer.position.x = Position::End;
    //     layer.scale = 3.00;

    // let res = canvas.to_image();
    // res.write_to_png("output15.png", true);

    use images_editor::formats::svg::read_svg_from_path;

    let mut canvas = Canvas::new(Resolution::new(1000, 1000));
    canvas.read_svg_from_path("bird.svg");

    canvas.to_image().write_to_png("output16.png", true);

    return ();
}
