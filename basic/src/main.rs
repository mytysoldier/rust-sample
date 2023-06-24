use std::fs::File;
use std::io::Write;

fn main() {
    let path = "sample.txt";
    let mut file = File::create(path).expect("file not found.");
    writeln!(file, "hello rust world.").expect("cannot write.");
}
