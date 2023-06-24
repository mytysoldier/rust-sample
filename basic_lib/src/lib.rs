pub mod random {
    use rand::{thread_rng, Rng};

    pub struct Dice {
        pub max: i32,
    }
    impl Dice {
        pub fn get(&self) -> i32 {
            let mut rng = thread_rng();
            rng.gen_range(0..=10)
        }
        pub fn fill(&self, data: &mut Vec<i32>) {
            for it in data {
                *it = self.get();
            }
        }
    }
}
