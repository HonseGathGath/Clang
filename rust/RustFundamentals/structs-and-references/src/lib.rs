// 1. Define the struct
pub struct TextFinder<'a> {
    pub text: &'a str,
}

// 2. Implement the struct and define the methods
impl<'a> TextFinder<'a> {
    pub fn new(txt: &'a str) -> Self {
        TextFinder { text: txt }
    }

    pub fn find_first(&'a self, keyword: &str) -> Option<&'a str> {
        self.text.lines().find(|line| line.contains(keyword))
    }

    pub fn find_many(&'a self, keyword: &str) -> Vec<&'a str> {
        self.text
            .lines()
            .filter(|line| line.contains(keyword))
            .collect()
    }
}

// Example usage
pub fn main() {
    let text = "Rust is fast and memory-efficient.\nOwnership is key to Rust's safety.\nRustaceans love the borrow checker.";
    let finder = TextFinder::new(text);

    let first = finder.find_first("Rust");
    println!("{:?}", first); // Should print: Some("Rust is fast and memory-efficient.")

    let matches = finder.find_many("Rust");
    println!("{:?}", matches); // Should print: ["Rust is fast and memory-efficient.", "Ownership is key to Rust's safety."]
}
