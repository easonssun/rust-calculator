use std::io::{self, Write};

fn main() {
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line!");

        let input = input.trim();
        if input == "exit" {
            break;
        }

        match evaluate_expression(input) {
            Ok(result) => println!("Result: {}", result),
            Err(e) => println!("Error: {}", e),
        }
    }
}

fn evaluate_expression(expr: &str) -> Result<f64, String> {
    let tokens: Vec<&str> = expr.split_whitespace().collect();
    if tokens.len() != 3 {
        return Err(
            "Invalid expression. Expected format: <number> <operator> <number>".to_string(),
        );
    }
    let num1: f64 = tokens[0]
        .parse()
        .map_err(|_| "Invalid number".to_string())?;
    let operator = tokens[1];
    let num2: f64 = tokens[0]
        .parse()
        .map_err(|_| "Invalid number".to_string())?;

    match operator {
        "+" => Ok(num1 + num2),
        "-" => Ok(num1 - num2),
        "*" => Ok(num1 * num2),
        "/" => {
            if num2 == 0.0 {
                Err("Division by zero".to_string())
            } else {
                Ok(num1 / num2)
            }
        }
        _ => Err("Invalid operator. Supported operator: +, -, *, /".to_string()),
    }
}
