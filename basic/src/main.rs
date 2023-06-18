fn main() {
    let mut v = vec![1, 2, 3, 4, 5];
    v.insert(0, 10);
    println!("v.first is {:?}", v.first());
    v.insert(v.len(), 99);
    println!("v.last is {:?}", v.last());
}
