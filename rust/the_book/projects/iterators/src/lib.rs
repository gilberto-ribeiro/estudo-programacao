#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

impl Shoe {
    fn new(size: u32, style: &str) -> Self {
        Self {
            size: size,
            style: String::from(style),
        }
    }
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe::new(10, "sneaker"),
            Shoe::new(13, "sandal"),
            Shoe::new(10, "boot"),
        ];

        let in_my_size = shoes_in_size(shoes, 10);

        assert_eq!(in_my_size, vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ]);
    }
}