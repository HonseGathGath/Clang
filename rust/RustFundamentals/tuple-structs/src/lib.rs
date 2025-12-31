pub struct Rectangle(pub f32, pub f32); // 1. Finish the struct

pub fn area(rect: &Rectangle) -> f32 {
    rect.0 * rect.1
}
