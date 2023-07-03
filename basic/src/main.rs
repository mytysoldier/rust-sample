use std::{
    io::{Error, Read},
    thread,
    time::Duration,
};

fn task() {
    for i in 0..3 {
        println!("thread #1 count {}.", i);
        thread::sleep(Duration::from_millis(1000));
    }
}

fn main() -> Result<(), Error> {
    // let task = || {
    //     for i in 0..3 {
    //         println!("thread #1 count {}.", i);
    //         thread::sleep(Duration::from_millis(1000));
    //     }
    // };
    let handle = thread::spawn(task);
    println!("please wait.");
    handle.join().unwrap();
    println!("program end.");
    Ok(())
}
