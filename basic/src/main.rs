fn main() {
    let x = 100;
    println!("x is {}", x);
    {
        let x = 200;
        println!("x is {}", x);
    }
    println!("x is {}", x);
}
