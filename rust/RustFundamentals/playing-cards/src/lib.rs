pub enum Card {
    // Define the Card variants here
    King,
    Queen,
    Jack,
    Numbered(u8, String),
}

pub fn card_description(card: &Card) -> String {
    // Your code here...
    let result: String = match card {
        Card::King => String::from("King"),
        Card::Queen => String::from("Queen"),
        Card::Jack => String::from("Jack"),
        Card::Numbered(value, str) => format!("{} of {}", value, str),
    };

    result
}
