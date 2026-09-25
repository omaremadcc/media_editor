#![allow(unused)]
use images_editor::{Resolution, canvas, image::Image};

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
    let image = Image::read_from_bmp(&buffer).unwrap();
    let image2 = Image::read_from_bmp(&buffer).unwrap();
    let image3 = Image::read_from_bmp(&buffer).unwrap();
    let image4 = Image::read_from_bmp(&buffer).unwrap();
    let image5 = Image::read_from_bmp(&buffer).unwrap();
    let buffer_2 = std::fs::read("low_no_merge.bmp").expect("Failed to read image file");
    let image_low = Image::read_from_bmp(&buffer_2).unwrap();

    let mut canvas = canvas::Canvas::new(Resolution::new(0, 0));
    // let id_of_layer = canvas.add_image(image, None);
    let id_of_layer_2 = canvas.add_image(image_low, None);
    let id_of_layer_3 = canvas.add_image(image3, None);
    // let id_of_layer_4 = canvas.add_image(image4, None);
    // let id_of_layer_5 = canvas.add_image(image5, None);

    // let mut layer_1 = canvas.get_mut_layer(id_of_layer);
    let mut layer_2 = canvas.get_mut_layer(id_of_layer_2);
    // layer_2.image.change_exposure(200.0);
    let mut layer_3 = canvas.get_mut_layer(id_of_layer_3);
    // let mut layer_4 = canvas.get_mut_layer(id_of_layer_4);
    // // layer_4.image.change_hue(90.0);
    // let mut layer_5 = canvas.get_mut_layer(id_of_layer_5);
    // layer_5.image.change_exposure(2.0);
    // canvas.bring_layer_forward(id_of_layer_2);

    let id_of_line_layer = canvas.add_line(100, 400, 0, 200, 10);

    let final_image = canvas.to_image();
    final_image.write_to_bmp("output11.bmp").unwrap();

    return ();
}
