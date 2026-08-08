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
    let mut image = Image::read_from_bmp(&buffer).unwrap();
    let buffer_2 = std::fs::read("low_no_merge.bmp").expect("Failed to read image file");
    let mut image_2 = Image::read_from_bmp(&buffer_2).unwrap();

    let mut canvas = canvas::Canvas::new(Resolution::new(0, 0));
    let index_of_layer = canvas.add_image(image, None);
    let index_of_layer_2 = canvas.add_image(image_2, None);

    // let layer = canvas.layers.get_mut(index_of_layer).unwrap();
    let layer_2 = canvas.get_mut_layer(index_of_layer_2);
    layer_2.move_x_percentage(0.3);

    let layer_1 = canvas.get_mut_layer(index_of_layer);
    layer_1.image.mirror_image_vertically();

    let final_image = canvas.to_image();
    final_image.write_to_bmp("output10.bmp").unwrap();

    return ();
}
