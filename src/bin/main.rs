use images_editor::bmp::Bmp;


fn main() -> () {
    let image = Bmp::read_from_file("image.bmp").unwrap();
    image.write_to_file("output2.bmp").unwrap();
    // let binary_data: Vec<u8> = fs::read("output1.bmp").expect("Failed to read image file");
    // println!("Binary data length: {}", binary_data.len());
    // println!("{:?}", binary_data[..].to_vec());

    // for (index, chunk) in binary_data[..].chunks_exact(3).into_iter().enumerate() {
    //     println!("{index}: ({:?}, {:?}, {:?})", chunk[2], chunk[1], chunk[0]);
    // }
    return ();
}
