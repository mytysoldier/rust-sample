#[derive(Debug)]
struct Person {
    name: String,
    age: i32,
}

fn new_person(name: &str, age: i32) -> Person {
    let p = Person {
        name: String::from(name),
        age: age,
    };
    p
}

fn main() {
    let mut a = new_person("masu", 30);
    println!("a is {:?}", a);
    let mut x = &mut a;
    println!("x is {:?}", x);
    x.age = 0;
    x.name = String::from("kato");
    println!("x is {:?}", x);
    println!("a is {:?}", a);
}
