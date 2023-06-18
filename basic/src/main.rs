static mut PERSON_ID: i32 = 0;
fn main() {
    let mut people = Vec::<Person>::new();
    people.push(Person::new("masu", 10, "Tokyo"));
    people.push(Person::new("yama", 20, "Tokyo"));
    people.push(Person::new("sato", 30, "Tokyo"));
    for p in &people {
        println!("{:#?}", p);
    }
}

#[derive(Debug)]
struct Person {
    id: i32,
    name: String,
    age: i32,
    addr: String,
}

impl Person {
    fn new(name: &str, age: i32, addr: &str) -> Person {
        let id = unsafe {
            PERSON_ID += 1;
            PERSON_ID
        };
        Person {
            id: id,
            name: name.to_string(),
            age: age,
            addr: addr.to_string(),
        }
    }
    fn add_age(&mut self, n: i32) {
        self.age += n;
    }
}
