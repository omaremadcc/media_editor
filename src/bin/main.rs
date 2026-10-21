#![allow(unused)]
use images_editor::{
    Resolution,
    canvas::{self, Layer, LayerPosition, Position},
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
use ttf_parser::{Face, GlyphId, OutlineBuilder};

fn main() -> () {
    let mut canvas = canvas::Canvas::new(Resolution::new(0, 0));
    let buffer = std::fs::read("png.png").unwrap();
    let image = Image::read_from_png(&buffer).unwrap();

    canvas.add_image(image);

    let svg_path = String::from(
        "
        M3 2a1 1 0 0 0-1 1v10a1 1 0 0 0 1 1h14a1 1 0 0 0 1-1V3a1 1 0 0 0-1-1H3zm0-2h14a3 3 0 0 1 3 3v10a3 3 0 0 1-3 3H3a3 3 0 0 1-3-3V3a3 3 0 0 1 3-3z
				"
    );
    let layer_id = canvas.add_svg_path(&svg_path, 1, Colors::white(), Colors::yellow());
    let layer = canvas.get_mut_layer(layer_id);
    // layer.position.x = Position::End;

    let text_layer_id = canvas.add_text(
        String::from("omar\nemad"),
        50,
        1,
        Colors::green(),
        Colors::orange(),
    );

    let text_layer = canvas.get_mut_layer(text_layer_id);
    text_layer.position.y = Position::End;

    let circle_layer_id = canvas.add_circle(100., 1, Colors::red(), Colors::blue());

    let id = canvas.add_rectangle(200, 200, 1, Colors::red(), Colors::blue());
    let layer = canvas.get_mut_layer(id);
    layer.position.x = Position::End;
    layer.scale = 3.0;

    let res = canvas.to_image();
    res.write_to_png("output15.png", true);
    return ();
}

fn extract_glyph(font_data: &[u8], character: char) -> Option<(GlyphPath, f32, f32)> {
    let face = Face::parse(font_data, 0).ok()?;

    let glyph_id = face.glyph_index(character)?;

    let advance_width = face.glyph_hor_advance(glyph_id)? as f32;

    let mut path = GlyphPath {
        commands: Vec::new(),
    };

    face.outline_glyph(glyph_id, &mut path);

    Some((path, advance_width, face.units_per_em() as f32))
}

struct GlyphPath {
    commands: Vec<PathCommand>,
}

impl OutlineBuilder for GlyphPath {
    fn move_to(&mut self, x: f32, y: f32) {
        self.commands.push(PathCommand::MoveTo((x, y)));
    }

    fn line_to(&mut self, x: f32, y: f32) {
        self.commands.push(PathCommand::LineTo((x, y)));
    }

    fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
        self.commands
            .push(PathCommand::QuadraticTo((x, y), (x1, y1)));
    }

    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
        self.commands
            .push(PathCommand::CubicTo((x, y), (x1, y1), (x2, y2)));
    }

    fn close(&mut self) {
        self.commands.push(PathCommand::Close);
    }
}
