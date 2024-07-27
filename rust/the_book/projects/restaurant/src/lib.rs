const PI: f32 = 3.14159265358979324;

mod back_of_house;
mod front_of_house;

use crate::front_of_house::hosting;

mod customer {
    use crate::back_of_house;
    // use super::front_of_house::{hosting, hosting::add_to_waitlist};
    use super::front_of_house::serving::*;
    use crate::front_of_house::hosting::{self, add_to_waitlist};

    fn eat_at_restaurant() {
        crate::front_of_house::hosting::add_to_waitlist(); // Absolute path
        super::front_of_house::hosting::add_to_waitlist(); // Relative path
        super::hosting::add_to_waitlist(); // Employing use keyword

        let mut meal = back_of_house::Breakfast::summer("Rye");
        meal.toast = String::from("Wheat");
        println!("I'd like {} toast please.", meal.toast);

        // meal.seasonal_fruit = String::from("blueberries");

        let order1 = back_of_house::Appetizer::Soup;
        let order2 = back_of_house::Appetizer::Salad;
    }

    pub fn deliver_order() {
        add_to_waitlist();
        hosting::add_to_waitlist();
        take_order();
        serve_order();
    }
}
