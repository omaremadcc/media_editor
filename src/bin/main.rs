#![allow(unused)]
use images_editor::{Resolution, canvas::Canvas, colors::Colors};

fn main() -> () {
    let mut canvas = Canvas::new(4000, 4000);

    let rect = canvas.add_rectangle(400, 400, 0, None, Colors::red(), 0f32);
    canvas.export_to_png("output16", true);

    use std::fs;

    fn get_installed_fonts() -> Vec<String> {
        let fonts_dir = r"C:\Windows\Fonts";

        fs::read_dir(fonts_dir)
            .unwrap()
            .filter_map(|entry| {
                let path = entry.ok()?.path();

                let extension = path.extension()?.to_str()?;

                if extension.eq_ignore_ascii_case("ttf") || extension.eq_ignore_ascii_case("otf") {
                    Some(path.to_string_lossy().into_owned())
                } else {
                    None
                }
            })
            .collect()
    }

    println!("{:?}", get_installed_fonts());

    return ();
}
