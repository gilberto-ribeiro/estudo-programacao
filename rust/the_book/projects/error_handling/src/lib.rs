pub mod guessing_game {
    pub struct Guess {
        value: i32,
    }

    impl Guess {
        pub fn new(value: i32) -> Guess {
            if value < 1 || value > 100 {
                panic!("Guess must be between 1 and 100. Got a value of {value}.")
            }
            Guess {value}
        }

        pub fn value(&self) -> i32 {
            self.value
        }
    }
}