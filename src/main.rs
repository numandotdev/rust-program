use std::io;

fn main() {
    let mut input = String::new();
    println!("Welcome to Currency Converter Console App");
    println!("Enter your amount $");
    
    io::stdin().read_line(&mut input).expect("Wrong Input");

    let dollar: f32 = input
        .trim()
        .parse()
        .expect("Failed to parse string into float");

    let inr: f32 = dollar * 83.44;

    println!("${}  is ₹{}", dollar, inr);
}
