// main.rs

extern crate my_project_name;

use my_project_name::currency;
use my_project_name::maths::maths_program;

fn main() {
    // Use the function from currency.rs
    // let amount_in_eur = 50.0;
    // let amount_in_usd = currency::convert_to_usd(amount_in_eur);

    // println!("Amount in USD: {}", amount_in_usd);

    // Use the function from maths_program.rs
    let number = 10;
    if maths_program::is_even(number) {
        println!("{} is even", number);
    } else {
        println!("{} is odd", number);
    }
}
