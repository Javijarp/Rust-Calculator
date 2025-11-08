pub fn is_valid_operator(operator: char) -> bool {
    matches!(operator, '+' | '-' | '*' | '/')
}
