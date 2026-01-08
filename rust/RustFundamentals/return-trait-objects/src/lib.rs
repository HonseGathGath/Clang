pub trait Speakable {
    fn speak(&self) -> String;
}

pub struct Dog {
    pub name: String,
    pub breed: String,
}

pub struct Robot {
    pub model: String,
    pub purpose: String,
}

impl Speakable for Dog {
    fn speak(&self) -> String {
        "Woof".to_string()
    }
}

impl Speakable for Robot {
    fn speak(&self) -> String {
        "Beep boop".to_string()
    }
}

pub fn get_speaker(kind: &str) -> Box<dyn Speakable> {
    match kind {
        "dog" => Box::new(Dog {
            name: "".to_string(),
            breed: "".to_string(),
        }),
        "robot" => Box::new(Robot {
            model: "".to_string(),
            purpose: "".to_string(),
        }),
        _ => panic!("Unknown speaker type"),
    }
}
// Example usage
pub fn main() {
    let dog_speaker = get_speaker("dog");
    println!("{}", dog_speaker.speak()); // Expected output: Woof

    let robot_speaker = get_speaker("robot");
    println!("{}", robot_speaker.speak()); // Expected output: Beep boop
}
