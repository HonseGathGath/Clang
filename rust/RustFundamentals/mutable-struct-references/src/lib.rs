// 1. Finish the struct definition
pub struct MutableTextFinder<'a> {
    text: &'a mut String,
}

impl<'a> MutableTextFinder<'a> {
    pub fn new(text: &'a mut String) -> Self {
        Self { text }
    }

    pub fn find_first(&self, keyword: &str) -> Option<&str> {
        self.text.lines().find(|line| line.contains(keyword))
    }

    pub fn replace_lines(&mut self, keyword: &str, replacement: &str) {
        let new_text: String = self
            .text
            .lines()
            .map(|line| {
                if line.contains(keyword) {
                    replacement
                } else {
                    line
                }
            })
            .collect::<Vec<_>>()
            .join("\n");

        *self.text = new_text;
    }

    pub fn get_text(&self) -> &str {
        self.text
    }
}
// Example usage
pub fn main() {
    let mut text = String::from("Rust is awesome\nLearning Rust\nFun with Rustaceans");
    let mut finder = MutableTextFinder::new(&mut text);

    let first = finder.find_first("Rust");
    println!("{:?}", first); // Should print: Some("Rust is awesome")

    finder.replace_lines("Rust", "Programming in Rust");
    println!("{}", finder.get_text()); // Should print the modified text
}
