pub struct Book {
    pub title: String,
    pub author: String,
    pub year: isize,
    pub likes: isize,
}

impl Book {
    pub fn new(title: &str, author: &str, year: isize) -> Book {
        Book {
            title: String::from(title),
            author: String::from(author),
            year,
            likes: 0,
        }
    }
}
