fn main() {
    let a = [1, 2, 3, 4, 5];
    let b = a.iter().find(|&&x| x == 3);
    let c = a.iter().find(|&&x| x > 10);
    println!("b is {:?}", b);
    println!("c is {:?}", c);
}
