pub fn calculate_area() -> u32 {
    // 1. Declare a variable named width
    let width: u32 = 10;
    // 2. Declare a variable named height
    let height: u32 = 10;
    let mult: u32 = width * height;
    // 3. Run the `prints_values` function with the width and height variables
    prints_values(width, height);
    // 4. Return the multiplication of width and height
    mult
}

// WARNING: Do not modify this function
pub fn prints_values(width: u32, height: u32) {
    println!("The width is: {}", width);
    println!("The height is: {}", height);
}
