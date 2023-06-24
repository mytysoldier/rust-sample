#[derive(Debug)]
struct Person {
    name: String,
    age: i32,
}

fn main() {
    let x: Person;
    {
        let a = Person {
            name: String::from("masu"),
            age: 30,
        };
        x = a;
    }
    println!("x is {:?}", x);
}
