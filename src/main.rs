use std::{ /*any::{type_name, Any}, io,*/ vec };

// mod currency;

// use currency::currency_tool;
// #[path = "./maths/maths_program.rs"]
// mod maths;

// use maths::is_even;

fn main() {
    // let result = currency_tool();
    // println!("{}", result);

    let user_data = ["Numan", "Kawsar", "Rezwan", "John", "Elisa"];

    // println!("{:?}", users)
    // for user in users {
    //     println!("{}", user)
    // }

    let user = User::new_user("Tony", 23);
    user.display_user();
}

struct User {
    name: String,
    age: u32,
}

impl User {
    fn new_user(name:&str, age: u32) -> User {
        User {
            name: String::from(name),
            age: age,
        }
    }
    fn display_user(&self) {
        println!("Name: {}, Age: {}", self.name, self.age);
    }    
}

