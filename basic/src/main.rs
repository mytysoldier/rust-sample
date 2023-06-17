fn main() {
    let lang = LANG::CHINESE;
    let m = match lang {
        LANG::JAPANESE => "日本語",
        _ => "any",
    };
    println!("lang is {}", m);
}

#[derive(Debug)]
enum LANG {
    JAPANESE = 81,
    ENGLISH = 82,
    CHINESE = 83,
    FRANCH = 84,
}
