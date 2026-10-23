use crate::formats::svg::SVGData;
use crate::graphic::{Graphic, GraphicType};
use crate::image::Image;

#[derive(Debug)]
pub enum Element {
    Image(Image),
    Graphic(Graphic),
    SVG(SVGData),
}

impl Element {
    pub fn is_image(&self) -> bool {
        matches!(self, Element::Image(_))
    }

    pub fn print_type(&self) {
        match self {
            Element::Image(_) => println!("Image"),
            Element::Graphic(graphic) => match graphic.graphic_type {
                GraphicType::Line { .. } => {
                    println!("Line");
                }
                GraphicType::Rectangle { .. } => {
                    println!("Rectangle");
                }
                GraphicType::Path { .. } => {
                    println!("Path")
                }
                GraphicType::Text { .. } => {
                    println!("Text")
                }
                GraphicType::Circle { .. } => {
                    println!("Circle")
                }
            },
            Self::SVG(..) => {
                println!("svg");
            }
        }
    }
}
