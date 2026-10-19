#[derive(Debug)]
pub struct Graphic {
    pub graphic_type: GraphicType,
    pub stroke_width: usize,
    pub stroke_color: u32,
    pub fill_color: u32,
}

#[derive(Debug)]
pub enum GraphicType {
    Line {
        start: (usize, usize),
        end: (usize, usize),
    },
    Rectangle {
        width: usize,
        height: usize,
        start: (usize, usize),
    },
    Path {
        commands: Vec<PathCommand>,
    },
}

#[derive(Debug)]
pub enum PathCommand {
    MoveTo((f32, f32)),
    LineTo((f32, f32)),
    QuadraticTo((f32, f32), (f32, f32)),
    CubicTo((f32, f32), (f32, f32), (f32, f32)),
    Close,
}
