use crate::canvas::{Canvas, Element, Layer};
use crate::graphic::{Graphic, GraphicType};

impl Canvas {
    pub fn add_line(
        &mut self,
        x0: usize,
        y0: usize,
        x1: usize,
        y1: usize,
        width: usize,
        stroke_color: Option<u32>,
    ) -> usize {
        let element = Element::Graphic(Graphic {
            graphic_type: GraphicType::Line {
                start: (x0, y0),
                end: (x1, y1),
            },
            stroke_width: width,
            stroke_color,
            fill_color: None,
        });
        self.add_layer(Layer::from_element(element, self.current_layer_id));
        return self.current_layer_id - 1;
    }
    pub fn add_rectangle(
        &mut self,
        width: usize,
        height: usize,
        stroke_width: usize,
        stroke_color: Option<u32>,
        fill_color: Option<u32>,
    ) -> usize {
        let element = Element::Graphic(Graphic {
            graphic_type: GraphicType::Rectangle { width, height },
            stroke_width,
            stroke_color,
            fill_color,
        });
        self.add_layer(Layer::from_element(element, self.current_layer_id));
        return self.current_layer_id - 1;
    }

    pub fn add_text(
        &mut self,
        text: String,
        size: usize,
        stroke_width: usize,
        stroke_color: Option<u32>,
        fill_color: Option<u32>,
    ) -> usize {
        let element = Element::Graphic(Graphic {
            graphic_type: GraphicType::Text { size, text },
            stroke_width,
            stroke_color,
            fill_color,
        });
        let layer = Layer::from_element(element, self.current_layer_id);
        self.add_layer(layer);
        return self.current_layer_id - 1;
    }

    pub fn add_circle(
        &mut self,
        radius: f32,
        stroke_width: usize,
        stroke_color: Option<u32>,
        fill_color: Option<u32>,
    ) -> usize {
        let layer = Layer::from_element(
            Element::Graphic(Graphic {
                graphic_type: GraphicType::Circle { radius },
                stroke_width,
                stroke_color,
                fill_color,
            }),
            self.current_layer_id,
        );
        self.add_layer(layer);

        return self.current_layer_id - 1;
    }
}
