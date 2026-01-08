use std::cmp::Ordering;

pub fn longest<'a>(first_slice: &'a str, second_slice: &'a str) -> &'a str {
    match first_slice
        .chars()
        .count()
        .cmp(&second_slice.chars().count())
    {
        Ordering::Less => second_slice,
        _ => first_slice,
    }
} // Finish the function

// Example usage
pub fn main() {
    let s1 = "short";
    let s2 = "longer string";

    let result = longest(s1, s2);
    println!("The longest string is: {}", result);
    assert_eq!(result, "longer string");

    let s3 = "equal";
    let s4 = "equal";
    let result = longest(s3, s4);
    println!("The longest string is: {}", result);
    assert_eq!(result, "equal");
}
