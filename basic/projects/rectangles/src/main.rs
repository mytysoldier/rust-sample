use std::{
    fs::File,
    io::{self, Read},
};

fn main() {
    let f = read_username_from_file();
    match f {
        Ok(_) => (),
        Err(e) => panic!("error occured. {:?}", e),
    }
}

fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;
    // let mut f = File::open("hello.txt")?;
    // let mut s = String::new();
    // f.read_to_string(&mut s)?;
    Ok(s)
    // let f = File::open("hello.txt");

    // let mut f = match f {
    //     Ok(file) => file,
    //     Err(e) => return Err(e),
    // };

    // let mut s = String::new();

    // match f.read_to_string(&mut s) {
    //     Ok(_) => Ok(s),
    //     Err(e) => Err(e),
    // }
}
