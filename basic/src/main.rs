use std::{
    fs::{self, File},
    io::Read,
};

fn main() {
    let path = "sample.txt";
    println!("read all lines by buffer.");
    let mut file = File::open(path).expect("file not found.");
    let mut data = String::new();
    file.read_to_string(&mut data).expect("read error");
    println!("data is {}", data);
}
