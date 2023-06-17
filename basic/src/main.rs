fn main() {
    let x = Some(10);
    match x {
        Some(i) => println!("i is {}", i),
        _ => (),
    };

    if let Some(i) = x {
        println!("i is {}", i);
    }
}
