fn main() {
//    loop {
//        println!("again!");
//    }

    let mut counter: usize = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}
