extern crate word_count;
fn main() {
    let counts = word_count::word_count("Hello_Hello_World How,are 1you1()doing.");
    println!("counts {:?}", counts);
}
