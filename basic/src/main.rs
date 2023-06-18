fn main() {
    let a = vec![1, 2, 3];
    let b = vec![4, 5];
    let v = [a, b].concat();
    println!("v.len is {}", v.len());
    for i in v {
        print!("{} ", i);
    }
    println!("");
}
