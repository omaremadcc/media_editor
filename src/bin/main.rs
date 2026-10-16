#![allow(unused)]
use images_editor::{
    Resolution,
    canvas::{self, Element, Layer, Position},
    colors::Colors,
    graphic::{
        Graphic, GraphicType,
        PathCommand::{LineTo, MoveTo, QuadraticTo},
    },
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
    // small_image_layer.set_x_position_end();
    // small_image_layer.set_y_position_end();
    small_image_layer.scale_layer(5.);
    // small_image_layer.scale_layer(1.);
    small_image_layer.opacity = 0.;

    // let line_id = canvas.add_line(30, 500, 200, 100, 5, Colors::rose());
    // let line_layer = canvas.get_mut_layer(line_id);
    // line_layer.change_exposure(3.);

    // let rectangle_id = canvas.add_rectangle(100, 100, 200, 200, 5, Colors::blue(), Colors::rose());
    // let rectangle_layer = canvas.get_mut_layer(rectangle_id);
    // rectangle_layer.change_exposure(3.);

    // canvas.add_line(0, 0, 235, 372, 1, Colors::orange());
    // canvas.add_line(235, 372, 800, 800, 1, Colors::purple());

    let buffer = std::fs::read("low_png.png").unwrap();
    let small_image = Image::read_from_png(&buffer).unwrap();
    let small_image_id = canvas.add_image(small_image);
    let small_image_layer = canvas.get_mut_layer(small_image_id);
    small_image_layer.position.x = Position::Point(300);
    small_image_layer.position.y = Position::Point(400);

    let graphic = Graphic {
        graphic_type: GraphicType::Path {
            commands: vec![
                MoveTo((200, 300)),
                QuadraticTo((300, 400), (390, 700)),
                LineTo((400, 300)),
                LineTo((700, 500)),
            ],
        },
        stroke_width: 2,
        stroke_color: Colors::rose(),
        fill_color: Colors::blue(),
    };
    let path = Element::Graphic(graphic);

    let layer = Layer::from_element(path, 100);
    canvas.add_layer(layer);

    let res = canvas.to_image();
    res.write_to_png("output15.png", true);
    return ();
}
