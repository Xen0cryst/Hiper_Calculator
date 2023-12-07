use rust_decimal::prelude::*;
use std::io;

fn main() {
    loop {
        println!("Hyper Scientific Calculator");
        println!("1. Add");
        println!("2. Subtract");
        println!("3. Multiply");
        println!("4. Divide");
        println!("5. Square Root");
        println!("6. Power");
        println!("7. Exit");

        let choice = get_user_input("Enter your choice: ");
        
        match choice {
            1 => perform_operation("Addition", |a, b| a + b),
            2 => perform_operation("Subtraction", |a, b| a - b),
            3 => perform_operation("Multiplication", |a, b| a * b),
            4 => perform_operation("Division", |a, b| a / b),
            5 => perform_unary_operation("Square Root", |a| a.sqrt()),
            6 => perform_binary_operation("Power", |a, b| a.powf(b)),
            7 => break,
            _ => println!("Invalid choice. Please enter a valid option."),
        }
    }
}

fn get_user_input(prompt: &str) -> usize {
    loop {
        println!("{}", prompt);

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input.");

        match input.trim().parse() {
            Ok(value) => return value,
            Err(_) => println!("Invalid input. Please enter a number."),
        }
    }
}

fn perform_operation<F>(operation_name: &str, operation: F)
where
    F: Fn(Decimal, Decimal) -> Decimal,
{
    let num1 = Decimal::from_str(&get_user_input("Enter the first number: ")).unwrap();
    let num2 = Decimal::from_str(&get_user_input("Enter the second number: ")).unwrap();

    let result = operation(num1, num2);
    println!("{}: {} = {}", operation_name, num1, result);
}

fn perform_unary_operation<F>(operation_name: &str, operation: F)
where
    F: Fn(Decimal) -> Decimal,
{
    let num = Decimal::from_str(&get_user_input("Enter the number: ")).unwrap();

    let result = operation(num);
    println!("{}: {} = {}", operation_name, num, result);
}

fn perform_binary_operation<F>(operation_name: &str, operation: F)
where
    F: Fn(Decimal, Decimal) -> Decimal,
{
    let base = Decimal::from_str(&get_user_input("Enter the base: ")).unwrap();
    let exponent = Decimal::from_str(&get_user_input("Enter the exponent: ")).unwrap();

    let result = operation(base, exponent);
    println!("{}: {}^{} = {}", operation_name, base, exponent, result);
}
