use images_editor::{canvas::Canvas, colors::Colors};

fn rectangle(canvas: &mut Canvas, width: usize, height: usize, x: i32, y: i32, color: u32) {
    let id = canvas.add_rectangle(width, height, 0, None::<u32>, Some(color), 0.0);

    let layer = canvas.get_mut_layer(id);
    layer.move_to_point_x(x);
    layer.move_to_point_y(y);
}

fn circle(canvas: &mut Canvas, radius: f32, x: i32, y: i32, color: u32) {
    let id = canvas.add_circle(radius, 0, None::<u32>, Some(color));

    let layer = canvas.get_mut_layer(id);
    layer.move_to_point_x(x);
    layer.move_to_point_y(y);
}

fn text(canvas: &mut Canvas, value: &str, size: usize, x: i32, y: i32, color: u32) {
    let id = canvas.add_text(value.to_string(), size, 0, None::<u32>, Some(color));

    let layer = canvas.get_mut_layer(id);
    layer.move_to_point_x(x);
    layer.move_to_point_y(y);
}

fn main() {
    let mut canvas = Canvas::new(1080, 1350);

    let black = Colors::black();
    let white = Colors::white();
    let orange = Colors::orange();
    let purple = Colors::purple();
    let rose = Colors::rose();

    // Background
    rectangle(&mut canvas, 1080, 1350, 0, 0, white);

    // Header
    text(&mut canvas, "FORM / 001", 25, 65, 55, black);
    text(
        &mut canvas,
        "CULTURE  /  DESIGN  /  IDEAS",
        20,
        65,
        100,
        black,
    );

    rectangle(&mut canvas, 950, 3, 65, 155, black);

    // Main geometric composition
    circle(&mut canvas, 350.0, 650, 210, orange);

    circle(&mut canvas, 185.0, 775, 390, purple);

    circle(&mut canvas, 80.0, 870, 480, rose);

    // Geometric interruptions
    rectangle(&mut canvas, 260, 24, 680, 355, black);
    rectangle(&mut canvas, 24, 380, 900, 245, black);

    rectangle(&mut canvas, 150, 150, 755, 560, white);

    // Main headline
    text(&mut canvas, "THE", 145, 65, 405, black);
    text(&mut canvas, "ART", 210, 65, 535, black);
    text(&mut canvas, "OF", 145, 65, 735, black);
    text(&mut canvas, "SEEING.", 125, 65, 865, black);

    // Editorial details
    rectangle(&mut canvas, 950, 3, 65, 1040, black);

    text(
        &mut canvas,
        "A STUDY IN VISUAL PERCEPTION",
        25,
        65,
        1070,
        black,
    );

    text(&mut canvas, "FORM FOLLOWS ATTENTION.", 28, 65, 1115, orange);

    text(
        &mut canvas,
        "THE WORLD IS NOT WHAT YOU SEE.",
        22,
        65,
        1200,
        black,
    );
    text(
        &mut canvas,
        "IT IS HOW YOU CHOOSE TO LOOK.",
        22,
        65,
        1235,
        black,
    );

    // Footer
    rectangle(&mut canvas, 950, 3, 65, 1285, black);

    text(
        &mut canvas,
        "INDEPENDENT VISUAL JOURNAL",
        18,
        65,
        1300,
        black,
    );
    text(&mut canvas, "VOL. 01  /  2026", 18, 790, 1300, black);

    canvas.export_to_png("editorial_poster.png", false).unwrap();
}
