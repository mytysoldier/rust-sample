fn main() {
    let v = [10, 20, 30, 40, 50];
    print(&v);
    let v = ['A', 'B', 'C', 'D', 'E'];
    print(&v);
    let v = ["one", "two", "three", "four", "five"];
    print(&v);
}

fn print<T>(a: &[T])
where
    T: std::fmt::Debug,
{
    print!("a is ");
    for i in a {
        print!("{:?} ", i);
    }
    println!("");
}
