use images_editor::canvas::Canvas;

const SIZE: usize = 1200;

fn place(canvas: &mut Canvas, id: usize, x: usize, y: usize) {
    let layer = canvas.get_mut_layer(id);
    layer.move_to_point_x(x as i32);
    layer.move_to_point_y(y as i32);
}

fn rect(canvas: &mut Canvas, x: usize, y: usize, w: usize, h: usize, fill: u32) {
    let id = canvas.add_rectangle(w, h, 0, Option::<u32>::None, Some(fill), 0.0);
    place(canvas, id, x, y);
}

fn circle(canvas: &mut Canvas, x: usize, y: usize, radius: f32, fill: u32) {
    let id = canvas.add_circle(radius, 0, Option::<u32>::None, Some(fill));
    place(canvas, id, x, y);
}

fn line(canvas: &mut Canvas, x0: usize, y0: usize, x1: usize, y1: usize, width: usize, color: u32) {
    let id = canvas.add_line(x0, y0, x1, y1, width, Some(color));

    // Lines already have endpoint coordinates.
    // Don't reposition them unless your line positioning
    // implementation explicitly supports it.
    let _ = id;
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut canvas = Canvas::new(SIZE, SIZE);

    let paper = 0xF2E9D8;
    let ink = 0x17233B;
    let blue = 0x3155C7;
    let red = 0xE64B35;
    let yellow = 0xF2BD42;
    let pale = 0xD5D9D0;

    // Background.
    rect(&mut canvas, 0, 0, SIZE, SIZE, paper);

    // Header rules.
    rect(&mut canvas, 72, 65, 1056, 3, ink);
    rect(&mut canvas, 72, 1120, 1056, 3, ink);

    // Large architectural blue block.
    rect(&mut canvas, 670, 160, 380, 650, blue);

    // Sun, overlapping the architectural block.
    circle(&mut canvas, 270, 220, 150.0, red);
    circle(&mut canvas, 270, 220, 105.0, yellow);

    // Repeated horizontal lines create a technical,
    // printmaking-inspired texture.
    for i in 0..24 {
        let y = 440 + i * 12;
        line(&mut canvas, 100, y, 530, y, 2, ink);
    }

    // Geometric architecture.
    rect(&mut canvas, 750, 245, 150, 360, paper);
    rect(&mut canvas, 900, 510, 100, 300, red);
    rect(&mut canvas, 670, 810, 380, 26, yellow);

    // A pale circular counterpoint.
    circle(&mut canvas, 500, 760, 105.0, pale);
    circle(&mut canvas, 500, 760, 63.0, ink);
    circle(&mut canvas, 500, 760, 28.0, paper);

    // Vertical axis.
    rect(&mut canvas, 555, 160, 3, 650, ink);

    // Small graphic accents.
    rect(&mut canvas, 105, 180, 16, 16, ink);
    rect(&mut canvas, 135, 180, 16, 16, red);
    rect(&mut canvas, 165, 180, 16, 16, blue);

    circle(&mut canvas, 1060, 875, 18.0, red);
    circle(&mut canvas, 1110, 875, 18.0, yellow);

    // Footer typography.
    let title = canvas.add_text(
        "FORM / STUDY 01".to_string(),
        42,
        0,
        Option::<u32>::None,
        Some(ink),
    );
    place(&mut canvas, title, 82, 910);

    let subtitle = canvas.add_text(
        "GEOMETRY IN MOTION".to_string(),
        18,
        0,
        Option::<u32>::None,
        Some(blue),
    );
    place(&mut canvas, subtitle, 85, 980);

    let edition = canvas.add_text(
        "INDEPENDENT GRAPHIC EXPERIMENT".to_string(),
        14,
        0,
        Option::<u32>::None,
        Some(ink),
    );
    place(&mut canvas, edition, 85, 1040);

    // Registration marks.
    for &(x, y) in &[(60, 60), (1125, 60), (60, 1125), (1125, 1125)] {
        line(&mut canvas, x - 8, y, x + 8, y, 2, ink);
        line(&mut canvas, x, y - 8, x, y + 8, 2, ink);
    }

    canvas.export_to_png("poster.png", false)?;

    Ok(())
}
