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
    MoveTo((usize, usize)),
    LineTo((usize, usize)),
    ClosePath,
    QuadraticTo((usize, usize), (usize, usize)),
    CubicTo((usize, usize), (usize, usize), (usize, usize)),
    Close,
}
