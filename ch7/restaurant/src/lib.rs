mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

pub fn eat_at_restaurant() {
    // Full path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    // seasonal_fruit is a private field
    // println!("I'd like {} toast please.", meal.toast);
    // meal.seasonal_fruit = String::from("blueberries");
}

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    // All variants are public
    pub enum Appetizer {
        Soup,
        Salad,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Self {
            Self {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    fn fix_incorrect_order() {
        cook_order();

        super::deliver_order();
    }

    fn cook_order() {}
}

fn deliver_order() {}


mod customer {
    use crate::front_of_house::hosting;

    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist();
    }
}

/*
// Always specify the parent module when calling the functions
use std::collections::HashMap;
let mut map = HashMap::new();

// Use the full path and when use stucts, enums, and other items
// Or use parent module when there are type name conflicts
use std::fmt;
use std::io;
fn func1() -> fmt::Result {}
fn func2() -> io::Result<()> {}

// We can specify a new local name using 'as'
use std::fmt::Result;
use std::io::Result as IoResult;
fn func1() -> Result {}
fn func2() -> IoResult<()> {}
*/
