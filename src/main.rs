use std::io;

fn main() {
    println!("=== Basic Calculator ===");
    
    loop {
        println!("\nChoose an operation:");
        println!("1. Addition (+)");
        println!("2. Subtraction (-)");
        println!("3. Multiplication (*)");
        println!("4. Division (/)");
        println!("5. Quit");
        
        let choice = read_input("Your choice (1-5): ");
        
        match choice.trim() {
            "1" => perform_operation(Operation::Addition),
            "2" => perform_operation(Operation::Subtraction),
            "3" => perform_operation(Operation::Multiplication),
            "4" => perform_operation(Operation::Division),
            "5" => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid choice. Please enter a number between 1 and 5."),
        }
    }
}

#[derive(Debug)]
enum Operation {
    Addition,
    Subtraction,
    Multiplication,
    Division,
}

fn perform_operation(op: Operation) {
    let num1 = read_number("Enter the first number: ");
    let num2 = read_number("Enter the second number: ");
    
    let result = match op {
        Operation::Addition => num1 + num2,
        Operation::Subtraction => num1 - num2,
        Operation::Multiplication => num1 * num2,
        Operation::Division => {
            if num2 == 0.0 {
                println!("Error: Division by zero is not allowed!");
                return;
            }
            num1 / num2
        }
    };
    
    let symbol = match op {
        Operation::Addition => "+",
        Operation::Subtraction => "-",
        Operation::Multiplication => "*",
        Operation::Division => "/",
    };
    
    println!("Result: {} {} {} = {}", num1, symbol, num2, result);
}

fn read_number(message: &str) -> f64 {
    loop {
        let input = read_input(message);
        
        match input.trim().parse::<f64>() {
            Ok(number) => return number,
            Err(_) => println!("Error: Please enter a valid number."),
        }
    }
}

fn read_input(message: &str) -> String {
    println!("{}", message);
    let mut input = String::new();
    
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    
    input
}