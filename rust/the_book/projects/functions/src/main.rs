fn main() {
//    another_function(5);
    printed_labeled_measurement(5, 'm');
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

fn printed_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}
