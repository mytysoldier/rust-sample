#[derive(Debug)]
struct Person {
    name: &'static str,
    age: i32,
}

fn print_a(a: &Person) {
    println!("print_a: a is {:?}", a);
}

fn main() {
    let a = Person {
        name: "masu",
        age: 10,
    };
    print_a(&a);
    println!("main: a is {:?}", a);
}
