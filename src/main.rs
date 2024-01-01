use std::{io, any::{type_name, Any}, result};

mod currency;

use currency::currency_tool;
// #[path = "./maths/maths_program.rs"]
// mod maths;

// use maths::is_even;

fn main() {
    let result = currency_tool();
    println!("{}", result);
}