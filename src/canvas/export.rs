use crate::canvas::Canvas;
use std::io::Error;

impl Canvas {
    pub fn export_to_png(&self, path: &str) -> Result<(), Error> {
        let image = self.to_image();
        image.write_to_png(path, true)
    }

    pub fn export_to_bmp(&self, path: &str) -> Result<(), Error> {
        let image = self.to_image();
        image.write_to_bmp(path, true)
    }
}
