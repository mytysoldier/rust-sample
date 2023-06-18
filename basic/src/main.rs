fn main() {
    let pa = Person {
        id: 100,
        name: String::from("masu"),
        age: 10,
        addr: String::from("Tokyo"),
    };
    print_person(&pa);
}

struct Person {
    id: i32,
    name: String,
    age: i32,
    addr: String,
}

fn print_person(pa: &Person) {
    println!("{}: {} ({}) in {}", pa.id, pa.name, pa.age, pa.addr);
}
