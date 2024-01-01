use std::io;

pub fn currency_tool () ->f32 {
    let mut curr = String::new();
    let mut curr_type = String::new();
   
    println!("Enter Your Amount $: ");
    
    io::stdin().read_line(& mut curr).expect("Failed to fetch the input");
    
    println!("Now Select Currency to convert (Select by numbers)");
    println!("1. INR");
    println!("2. EURO");
    println!("3. BTC");

    io::stdin().read_line(& mut curr_type).expect("Failed to fetch the input");

    return currency_converter(curr, curr_type);
}


pub fn currency_converter(input:String, convert_type:String) -> f32 {
    let dollar:f32 = input.trim().parse().expect("Failed to parse dollar!");
    let convert_to:i8 = convert_type.trim().parse().expect("Failed to parse convert type!");

    let mut to_curr: f32 = 0.00;

    match convert_to {
        1  => to_curr = 83.44, // inr
        2 => to_curr = 0.91, //euro
        3 => to_curr = 0.000024, //bitcoin
        _ => to_curr = 0.00, // default
    }


    if to_curr == 0.00 {
        println!("Oops! you have entered wrong convertion type");
        currency_converter(input, convert_type)
    } else {
        return dollar * to_curr;
    }
}

// pub fn convert_to_inr() {
//     let mut input = String::new();
//     println!("Welcome to Currency Converter Console App");
//     println!("Enter your amount $");

//     io::stdin().read_line(&mut input).expect("Wrong Input");

//     let dollar: f32 = input
//         .trim()
//         .parse()
//         .expect("Failed to parse string into float");

//     let inr: f32 = dollar * 83.44;

//     println!("${}  is ₹{}", dollar, inr);
// }