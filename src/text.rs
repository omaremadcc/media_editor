use crate::graphic::PathCommand;
use ttf_parser::OutlineBuilder;

pub struct GlyphPath {
    pub commands: Vec<PathCommand>,
}
impl GlyphPath {
    pub fn new() -> Self {
        GlyphPath {
            commands: Vec::new(),
        }
    }
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
