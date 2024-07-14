fn main() {
    let condition: bool = true;
    let number: i32 = if condition { 5 } else { 6 };

    println!("the value of the number is: {number}");
    println!(
        "the value of the number is: {}",
        if condition { "five" } else { "six" }
    );

    if condition {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    if number != 0 {
        println!("number was something other than zero");
    }

    if number % 5 == 0 {
        println!("number is divisible by 5");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 5, 3, or 2");
    }
}
