fn main() {
    let mut v = ["one", "two", "three", "four", "five"];
    v.sort();
    let x = v.join(" ");
    println!("x is {}", x);
    v.reverse();
    let x = v.join(" ");
    println!("x is {}", x);
}
