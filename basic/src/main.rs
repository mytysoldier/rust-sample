#[derive(Debug)]
struct Person {
    name: &'static str,
    age: i32,
}

fn print_a(a: &Person) {
    println!("print_a: a is {:?}", a);
}

fn move_a(a: Person) {}

fn add_age(a: &mut Person) {
    a.age += 1;
}

fn main() {
    let a = (100, "masu");
    println!("a is {:?}", a);
    let x = a;
    println!("x is {:?}", x);
    let y = a;
    println!("y is {:?}", y);
}
