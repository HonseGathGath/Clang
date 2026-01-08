// Finish the function
pub fn filter_starts_with<'a>(
    vec: &'a [String],
    prefix: &'a str,
) -> impl Iterator<Item = &'a String> {
    vec.iter().filter(move |c| c.starts_with(prefix))
}

// Example usage
pub fn main() {
    let input = vec![
        String::from("apple"),
        String::from("apricot"),
        String::from("banana"),
        String::from("cherry"),
    ];
    let filtered: Vec<&String> = filter_starts_with(&input, "ap").collect();
    println!("{:?}", filtered); // Expected output: ["apple", "apricot"]
}
