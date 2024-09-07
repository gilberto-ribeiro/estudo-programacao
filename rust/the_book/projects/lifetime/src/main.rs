use std::fmt::Display;

fn main() {
    let s: &'static str = "I have a static lifetime.";
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest_string(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {result}");

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();
    let _i = ImportantExcerpt {
        part: first_sentence,
    };
}
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
fn longest_string(x: &str, y: &str) -> String {
    if x.len() > y.len() {
        String::from(x)
    } else {
        String::from(y)
    }
}

fn returns_str(x: &str) -> &str {
    x
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {announcement}");
        self.part
    }
}

// fn longest_with_an_announcement<'a, T>(
//     x: &'a str,
//     y: &'a str,
//     ann: T,
// ) -> &'a str
// where
//     T: Display,
// {
//     println!("Announcement! {ann}");
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

// fn longest_with_an_announcement<'a>(x: &'a str, y: &'a str, ann: &impl Display) -> &'a str {
//     println!("Announcement! {ann}");
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

fn longest_with_an_announcement<'a, T: Display>(x: &'a str, y: &'a str, ann: T) -> &'a str {
    println!("Announcement! {ann}");
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
