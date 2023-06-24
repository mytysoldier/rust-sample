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
    let a = Person {
        name: "masu",
        age: 10,
    };
    let mut x = a;
    println!("x is {:?}", x);
    x.age += 1;
    println!("x is {:?}", x);
    add_age(&mut x);
    // a.age += 1;
    // println!("a is {:?}", a);
}
