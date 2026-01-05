use std::fs::File;
use std::io::{self, Read};

pub fn read_file(file_path: &str) -> Option<String> {
    // Hint: Use `File::open` and `.read_to_string()` with `?` to propagate errors.
    let mut file = File::open(file_path).ok()?;

    let mut file_to_string = String::new();

    file.read_to_string(&mut file_to_string).ok()?;

    Some(file_to_string)
}

// Example usage
pub fn main() {
    let file_path = "example.txt";

    match read_file(file_path) {
        Some(contents) => println!("File contents:\n{}", contents),
        None => println!("Failed to read the file."),
    }
}
