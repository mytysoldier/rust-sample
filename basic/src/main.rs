fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() <= 1 {
        panic!("パラメーターは必須です");
    } else {
        for (i, s) in args.iter().enumerate() {
            println!("{}: {}", i, s);
        }
    }
}
