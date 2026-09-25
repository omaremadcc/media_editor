pub struct Graphic {
    pub graphic_type: GraphicType,
    pub stroke_width: usize,
    pub stroke_color: u32,
    pub fill_color: u32,
}
pub enum GraphicType {
    Line {
        start: (usize, usize),
        end: (usize, usize),
    },
    // Rectangle {
    //     width: f32,
    //     height: f32,
    //     start: (f32, f32),
    // },
    // Circle {
    //     radius: f32,
    //     start: (f32, f32),
    // },
}
