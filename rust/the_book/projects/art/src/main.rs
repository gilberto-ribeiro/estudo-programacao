use art::{mix, PrimaryColor};
use std::process;

fn main() {
    let c_1 = PrimaryColor::Blue;
    let c_2 = PrimaryColor::Yellow;
    let c_3 = mix(&c_1, &c_2).unwrap_or_else(|err| {
        eprintln!("Error: {err}.");
        process::exit(1);
    });
    println!("{c_3:?}");
}
