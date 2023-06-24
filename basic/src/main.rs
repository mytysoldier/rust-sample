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
    let mut a = Person {
        name: "masu",
        age: 10,
    };
    println!("a is {:?}", a);
    add_age(&mut a);
    println!("a is {:?}", a);
}
