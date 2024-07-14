fn main() {
//    another_function(5);
    printed_labeled_measurement(five(), 'm');

    let x = plus_one(five());

    println!("The value of x is: {x}");
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

fn printed_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value} {unit_label}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
