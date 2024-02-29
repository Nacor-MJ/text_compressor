
fn main() {
    let og = "I am very proud of my country and its citizens";
    let compressed = text_compressor::CompressedArr::new(og);
    println!("Compressed: '{:#?}'\n Og: '{:#?}'", compressed, og);

    dbg!(std::mem::size_of_val(og));
    dbg!(std::mem::size_of_val(&compressed));
}
