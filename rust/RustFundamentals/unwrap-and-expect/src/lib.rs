use std::{env, fs::File, io::Read};

pub fn read_file_to_string(path: &str) -> String {
    // 1. Implement the function
    //
    let errMSG: &str = &format!("Failed to read file: {}", path);
    let mut file = File::open(path).expect(errMSG);
    let mut file_content: String = "".to_string();
    file.read_to_string(&mut file_content);

    file_content
}

pub fn get_env_variable(key: &str) -> String {
    let value: String = env::var(key).unwrap();

    value
}

/// Example usage
pub fn main() {
    // Example 1: Using read_file_to_string
    let file_content = read_file_to_string("example.txt");
    println!("File content: {}", file_content);

    // Example 2: Using get_env_variable
    std::env::set_var("EXAMPLE_KEY", "example_value");
    let value = get_env_variable("EXAMPLE_KEY");
    println!("Environment variable value: {}", value);

    // Must panic
    read_file_to_string("nonexistent.txt");
    get_env_variable("MISSING_KEY");
}
