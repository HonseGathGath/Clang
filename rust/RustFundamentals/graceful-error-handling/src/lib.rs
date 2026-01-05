pub fn parse_percentage(input: &str) -> Result<u8, String> {
    let percentage: u8 = input.parse().map_err(|_| "Invalid input".to_string())?;

    if percentage <= 100 {
        Ok(percentage)
    } else {
        Err("Percentage out of range".to_string())
    }

    // TODO: Implement the function here
}

// Example usage
pub fn main() {
    let result = parse_percentage("50");
    assert_eq!(result, Ok(50));

    let result = parse_percentage("101");
    assert_eq!(result, Err("Percentage out of range".to_string()));

    let result = parse_percentage("abc");
    assert_eq!(result, Err("Invalid input".to_string()));
}
