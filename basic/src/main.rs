struct Sample {
    x: i32,
}

impl Sample {
    fn new(x: i32) -> Self {
        Self { x }
    }
    fn inc(&self) -> i32 {
        self.x + 1
    }
    fn add(&self, x: i32) -> i32 {
        self.x + x
    }
}

fn main() {
    let a = Sample::new(10);
    let ans = a.inc();
    println!("ans is {}", ans);
    let ans = a.add(20);
    println!("ans is {}", ans);
}
