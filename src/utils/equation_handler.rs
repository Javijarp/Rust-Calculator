// The idea is to handle pemdas recursively
/*
1. Check for parenthesis
2. Check for exponents
3. Check for multiplication and division
4. Check for addition and subtraction
*/

/*
What is the basic plan here?
We want to divide the equation into smaller parts, and work on those parts separately.
For example, if we have the equation "3 + 5 * 2", we want
to first identify the multiplication, and then divide the equation into "3 + (5 * 2)".
Then we can handle the multiplication first, and then the addition.
*/

use crate::utils::helper::is_valid_operator;

pub fn handle_equation(equation: String) -> f64 {
    let mut eq = equation.trim().to_string();

    // Step 1: Handle parentheses first (innermost to outermost)
    loop {
        let mut open_idx: Option<usize> = None;
        let mut found = false;

        for (i, ch) in eq.char_indices() {
            if ch == '(' {
                // Track the most recent '(' for innermost logic
                open_idx = Some(i);
            } else if ch == ')' {
                if let Some(start) = open_idx {
                    // Extract and evaluate the inner expression
                    let inner = eq[start + 1..i].to_string();
                    println!("Evaluating parentheses: ({})", inner);
                    let result = handle_equation(inner);

                    // Replace the entire "(...)" with the result
                    eq.replace_range(start..=i, &result.to_string());
                    println!("After resolving parentheses: {}", eq);
                    found = true;
                    break;
                }
            }
        }

        if !found {
            break; // No more parentheses
        }
    }

    // Step 2: Find the operator with LOWEST precedence (scan right-to-left for same precedence)
    // Higher weight = higher precedence (handled later)
    let weights = |c: char| match c {
        '+' | '-' => 1,
        '*' | '/' => 2,
        '^' => 3, // Exponentiation has highest precedence
        _ => 0,
    };

    let mut lowest_op_pos: Option<usize> = None;
    let mut lowest_op: char = ' ';
    let mut lowest_weight = i32::MAX;

    // Scan from RIGHT to LEFT to handle left-associativity correctly
    // For exponents, this gives right-associativity (2^3^2 = 2^(3^2) = 512, not (2^3)^2 = 64)
    for (i, c) in eq.char_indices().rev() {
        if is_valid_operator(c) || c == '^' {
            let w = weights(c);
            // Pick operators with LOWER weight (+ and - before * and /, before ^)
            // For same weight, pick the rightmost (due to rev iterator)
            if w <= lowest_weight {
                lowest_weight = w;
                lowest_op = c;
                lowest_op_pos = Some(i);
            }
        }
    }

    // Step 3: If operator found, split and recurse
    if let Some(i) = lowest_op_pos {
        let left = eq[..i].trim().to_string();
        let right = eq[i + 1..].trim().to_string();

        println!(
            "Left: '{}', Right: '{}', Operator: '{}'",
            left, right, lowest_op
        );

        return match lowest_op {
            '+' => handle_equation(left) + handle_equation(right),
            '-' => handle_equation(left) - handle_equation(right),
            '*' => handle_equation(left) * handle_equation(right),
            '/' => {
                let r = handle_equation(right);
                if r == 0.0 {
                    panic!("Division by zero");
                }
                handle_equation(left) / r
            }
            '^' => {
                let base = handle_equation(left);
                let exponent = handle_equation(right);
                base.powf(exponent)
            }
            _ => 0.0,
        };
    }

    // Base case: parse as number
    let num = eq.parse::<f64>().unwrap_or(0.0);
    println!("Parsed base number: {}", num);
    num
}
