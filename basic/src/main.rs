struct Rectangle {
    width: f32,
    height: f32,
}
struct Triangle {
    base: f32,
    height: f32,
}
struct Circle {
    radius: f32,
}

trait CalcArea {
    fn calc_area(&self) -> f32;
}

impl CalcArea for Rectangle {
    fn calc_area(&self) -> f32 {
        self.width * self.height
    }
}

impl CalcArea for Triangle {
    fn calc_area(&self) -> f32 {
        self.base * self.height * 0.5
    }
}

impl CalcArea for Circle {
    fn calc_area(&self) -> f32 {
        self.radius * self.radius * 3.14
    }
}

fn main() {
    let rect = Rectangle {
        width: 10.0,
        height: 20.0,
    };
    let tri = Triangle {
        base: 10.0,
        height: 20.0,
    };
    let cir = Circle { radius: 10.0 };

    println!("rect area is {}", rect.calc_area());
    println!("tri area is {}", tri.calc_area());
    println!("cir area is {}", cir.calc_area());
}
