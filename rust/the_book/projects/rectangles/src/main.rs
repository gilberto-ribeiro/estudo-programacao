#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // fn area(&self) -> u32 {
    fn area(self: &Self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    #[warn(unused_variables)]
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn yes_or_no(var: bool) -> String {
    match var {
        true => String::from("yes"),
        false => String::from("no"),
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    if rect1.width() {
        println!(
            "The area of the rectangle is {} square pixels.",
            rect1.area()
        );
    }

    println!(
        "Can rect1 hold rect2? {}.",
        yes_or_no(rect1.can_hold(&rect2))
    );
    println!(
        "Can rect1 hold rect3? {}.",
        yes_or_no(rect1.can_hold(&rect3))
    );

    let sq1 = Rectangle::square(50);
    println!("Area of sq1: {}", sq1.area());
}
