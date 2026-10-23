use crate::formats::svg::SVGData;
use crate::graphic::Graphic;
use crate::image::Image;

#[derive(Debug)]
pub(crate) enum Element {
    Image(Image),
    Graphic(Graphic),
    SVG(SVGData),
}
