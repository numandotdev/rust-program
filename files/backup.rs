// #[path = "./maths/console_calc.rs"]
// mod maths;
// #[path = "./server/api.rs"]
// mod server;
// mod currency;

// use currency::currency_tool;
// use std::fs::File;
// use std::io::prelude::*;
// use std::fs;
// use maths::console_calculator;
// use maths::is_even;
// use reqwest;


// #[tokio::main] //for making the main asynchronous
fn main() {

    // let mut input1 = String::new();
    // let mut input2 = String::new();

    // println!("Enter Your First Value: ");
    // io::stdin().read_line(&mut input1).expect("Something went wrong!");
    // println!("Enter Your Second Value: ");
    // io::stdin().read_line(&mut input2).expect("Something went wrong!");

    // println!("Addition of 2 number is {}", console_calculator(input1, input2));

    // Specify the URL of the API you want to call
    // let api_url = "https://jsonplaceholder.typicode.com/todos/1";

    // Call the function to make the API request
    // if let Err(err) = make_api_request(api_url).await {
    //     eprintln!("Error: {}", err);
    // }
    // server::api_call("https://jsonplaceholder.typicode.com/todos");
    // const FILE_PATH: &str = "files/data.txt";
    // let _ = fs::write(FILE_PATH, "Data is changed!!!!");
    // let file_content =fs::read_to_string(FILE_PATH).expect("Unable to read file content!");
    // println!("{}", file_content);

    // --snip--
    //  println!("In file {}", "files/data.txt");

    //  let contents = fs::read_to_string("files/data.txt")
    //      .expect("Should have been able to read the file");
    //  println!("With text:\n{contents}");

    // let fruits = vec!["Banana", "Mango", "Apple", "Grapes", "Pinaple", "Orange"];
    // println!("{:?}", fruits);

    // for (index, fruit) in fruits.iter().enumerate() {
    //     println!("The name of the fruit is {} at index {}", fruit, index);
    // };

    // let result = currency_tool();
    // println!("{}", result);

    // println!("{:?}", users)
    // for user in users {
    //     println!("{}", user)
    // }

    // let user_data = ["Numan", "Kawsar", "Rezwan", "John", "Elisa"];
    // let user = User::new_user("Tony", 23);
    // user.display_user();

    // let x:i8 = 2;
    // print_value(x);
    // println!("{}", x);
    // let y = x + 1;
    // println!("y={}, x={}", y, x);
}


// async fn make_api_request(api_url: &str) -> Result<(), reqwest::Error> {
//     // Make a GET request to the API
//     let response = reqwest::get(api_url).await?;

//     // Check if the request was successful (status code 2xx)
//     if response.status().is_success() {
//         // Read the response body as a string
//         let body = response.text().await?;
//         println!("API Response: {}", body);
//     } else {
//         println!("API Request failed with status: {}", response.status());
//     }

//     Ok(())
// }

// fn print_value(value: i8) {
//     println!("The number is {}", value);
// }

// struct User {
//     name: String,
//     age: u32,
// }

// impl User {
//     fn new_user(name: &str, age: u32) -> User {
//         User {
//             name: String::from(name),
//             age: age,
//         }
//     }
//     fn display_user(&self) {
//         println!("Name: {}, Age: {}", self.name, self.age);
//     }
// }
