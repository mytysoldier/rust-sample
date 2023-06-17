fn main() {
    let x = None;
    let v = match x {
        Some(i) => i,
        None => -1,
    };
    println!("v is {}", v);
}
