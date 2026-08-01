use images_editor::bmp::Bmp;


fn main() -> () {
    let image = Bmp::read_from_file("low_no_merge.bmp").unwrap();
    println!("{:?}", image);
    return ();
}
