mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn _seat_at_table() {}
    }

    mod serving {
        fn _take_order() {}

        fn _serve_order() {}

        fn _take_payment() {}
    }
}

mod back_of_house {
    // For `enum` all elements are public if we use pub before `enum`
    pub enum Appetizer {
        Soup,
        Salad,
    }

    // For `struct` we need to make pub each element
    pub struct Breakfast {
        pub toast: String,
        _seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                _seasonal_fruit: String::from("peaches"),
            }
        }
    }

    fn _fix_incorrect_order() {
        _cook_order();
        super::_deliver_order();
    }

    fn _cook_order() {}
}

fn _deliver_order() {}

use crate::front_of_house::hosting;
use crate::front_of_house::hosting::add_to_waitlist;

use std::collections::HashMap;

pub fn eat_at_restaurant() {
    // ---------------------- Path ----------------------
    // `crate` means root

    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    // If we will do `use` above this function then we can go with below
    hosting::add_to_waitlist();
    // or if use is out of scope
    // super::hosting::add_to_waitlist();

    // If we specify whole path to the add_to_waitlist we can call directly
    add_to_waitlist();

    // HashMap
    let mut map = HashMap::new();
    map.insert(1, 2);

    // ---------------------- Struct ----------------------
    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it we're not allowed to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");

    // ---------------------- Enum ----------------------
    let _order1 = back_of_house::Appetizer::Soup;
    let _order2 = back_of_house::Appetizer::Salad;
}

// In below case not calling directly Result makes us able to explain to Rust which is from where
// use std::fmt;
// use std::io;

// fn function_one() -> fmt::Result {}
// fn function_two() -> io::Result<()> {}

// In case we would call Result like below we would fail
// use std::fmt::Result;
// use std::io::Result;

// We can also avoid that by providing new names with `as` keyword
// use std::fmt::Result;
// use std::io::Result as IoResult;

// fn function1() -> Result {}
// fn function2() -> IoResult<()> {}

// We can also make use `pub` -> this will do below for external users
// pub use crate::front_of_house::hosting;

// Before `pub use`
// restaurant::front_of_house::hosting::add_to_waitlist()

// After `pub use`
// restaurant::hosting::add_to_waitlist()

// We can call multiple libs from same crate as below
// use std::cmp::Ordering;
// use std::io;

// Instead of that we can do below
// use std::{cmp::Ordering, io};

// Below are the same too
// use std::io;
// use std::io::Write;

// &

// use std::io::{self, Write};

// Importing all items
// use std::collections::*;
