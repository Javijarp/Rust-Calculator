mod utils;

use crate::utils::equation_handler;

fn main() {
    let first_number = 0;
    let second_number = 10;
    let operator = '+';
    let equation_str = format!("{} {} {}", first_number, operator, second_number);

    println!(
        "Calculating the sum of {} and {}",
        first_number, second_number
    );

    let result = equation_handler::handle_equation("((5 + 5) * (5 * 5)) / (5^2 + 5^2)".to_string());
    println!("\nResult: {}", result);
}
