pub fn console_calculator(num1:String, num2:String) -> f32 {
    let a:f32 = num1.trim().parse().expect("Failed to parse the string");
    let b:f32 = num2.trim().parse().expect("Failed to parse the string");

    return a + b;
}