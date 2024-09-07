use tshirt_company::{self, Inventory, ShirtColor};

fn main() {

    let store = Inventory {
        shirts: vec![ShirtColor::Red, ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue, ShirtColor::Red],
    };

    let user_pref_1 = Some(ShirtColor::Blue);
    let giveaway_1 = store.giveaway(user_pref_1);
    println!("The user with preference {:?} gets {:?}", user_pref_1, giveaway_1);

    let user_pref_2 = None;
    let giveaway_2 = store.giveaway(user_pref_2);
    println!("The user with preference {:?} gets {:?}", user_pref_2, giveaway_2);
}
