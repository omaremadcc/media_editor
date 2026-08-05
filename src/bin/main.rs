use images_editor::{Pixel, image::Image};

fn main() -> () {
    // let image = images_editor::bmp::Bmp::read_from_file("low_no_merge.bmp").unwrap();
    // image.write_to_file("output2.bmp").unwrap();
    // let binary_data: Vec<u8> = fs::read("output1.bmp").expect("Failed to read image file");
    // println!("Binary data length: {}", binary_data.len());
    // println!("{:?}", binary_data[..].to_vec());

    // for (index, chunk) in binary_data[..].chunks_exact(3).into_iter().enumerate() {
    //     println!("{index}: ({:?}, {:?}, {:?})", chunk[2], chunk[1], chunk[0]);
    // }
    // let buffer = std::fs::read("low.bmp").expect("Failed to read image file");
    // let image = Image::read_from_bmp(&buffer).unwrap();
    // image.write_to_bmp("output5.bmp").unwrap();
    // println!("Buffer: {:?}", &buffer[..]);

    let buffer = std::fs::read("image.bmp").expect("Failed to read image file");
    let mut image = Image::read_from_bmp(&buffer).unwrap();

    // for (index, pixel) in image.pixels.iter().enumerate() {
    //     println!("index: {index}");
    //     println!("rgb({:?}, {:?}, {:?})", pixel.r, pixel.g, pixel.b);
    // }

    image.change_exposure(3f32);

    image.write_to_bmp("output7.bmp").unwrap();

    return ();
}
