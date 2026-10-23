use images_editor::{canvas::Canvas, colors::Colors};

fn main() {
    let mut canvas = Canvas::new(3840, 2924);

    canvas.add_rectangle(3840, 2924, 0, None, Colors::yellow());

    for (i, fill_color) in [
        Colors::white(),
        Colors::yellow(),
        Colors::blue(),
        Colors::green(),
        Colors::purple(),
        Colors::red(),
        Colors::orange(),
        Colors::black(),
    ]
    .iter()
    .enumerate()
    {
        let width = (3840 - 50) / 8;
        let padding = width * i + 25;
        let rect = canvas.add_rectangle(width, 1300, 0, None, *fill_color);
        canvas.get_mut_layer(rect).move_to_point_y(25);
        canvas.get_mut_layer(rect).move_to_point_x(padding as i32);
    }

    let rectangle1 = canvas.add_rectangle(3840 - 50, 1300, 20, Colors::black(), None);
    let layer = canvas.get_mut_layer(rectangle1);
    layer.move_to_point_y(25);
    layer.center_layer_x();

    let text = canvas.add_text(String::from("TEST"), 1000, 0, None, Colors::white());
    canvas.get_mut_layer(text).center_layer_x();
    canvas.get_mut_layer(text).move_layer_x_pixels(50);
    canvas.get_mut_layer(text).move_layer_y_pixels(75);
    canvas.get_mut_layer(text).move_layer_y_percent(0.1);

    let text = canvas.add_text(String::from("TEST"), 1000, 0, None, Colors::black());
    canvas.get_mut_layer(text).center_layer_x();
    canvas.get_mut_layer(text).move_layer_y_pixels(25);
    canvas.get_mut_layer(text).move_layer_y_percent(0.1);

    for i in (0..=10).rev() {
        let side = (3840 - 50 - 11 * 50) / 11;
        let padding_y = 1400;
        let padding_x = 25 + (side + 50) * i as i32;
        let rect = canvas.add_rectangle(side as usize, side as usize, 0, None, Colors::black());
        canvas.get_mut_layer(rect).move_to_point_y(padding_y);
        canvas.get_mut_layer(rect).move_to_point_x(padding_x);
        canvas
            .get_mut_layer(rect)
            .set_opacity((10 - i) as f32 / 10.0);
    }

    let _ = canvas.export_to_png("test1", false);
}
