use std::{
    fs::{self, File},
    io::{BufRead, BufReader, Read},
};

fn main() -> std::io::Result<()> {
    let path = "sample.txt";
    let file = File::open(path)?;
    for line in BufReader::new(file).lines() {
        println!("line is {}", line?);
    }
    Ok(())
}
