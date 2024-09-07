#[derive(Debug, PartialEq, Copy, Clone)]
pub enum ShirtColor {
    Blue,
    Red,
}

pub struct Inventory {
    pub shirts: Vec<ShirtColor>,
}

impl Inventory {
    pub fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    pub fn most_stocked(&self) -> ShirtColor {
        let mut num_blue = 0;
        let mut num_red = 0;

        for shirt in &self.shirts {
            match shirt {
                ShirtColor::Blue => num_blue += 1,
                ShirtColor::Red => num_red += 1,
            }
        }

        if num_blue >= num_red {
            ShirtColor::Blue
        } else {
            ShirtColor::Red
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn return_shirt_color_with_preference() {
        let user_preference = Some(ShirtColor::Red);
        let store = Inventory {
            shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
        };
        assert_eq!(ShirtColor::Red, store.giveaway(user_preference));
    }

    #[test]
    fn return_shirt_color_with_no_preference() {
        let user_preference = None;
        let store = Inventory {
            shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
        };
        assert_eq!(ShirtColor::Blue, store.giveaway(user_preference));
    }
}