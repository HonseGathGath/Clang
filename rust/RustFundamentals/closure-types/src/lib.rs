// 1. Based on the `main` function below,
// Find out the types of the closures and define them
pub fn create_typed_closures() -> (
    impl Fn(f64, f64) -> f64,
    impl FnMut(&mut f64, f64),
    impl FnOnce(String) -> String,
) {
    // 2. Implement calculate_total closure here
    let calculate_total = |price, tax_rate| price + price * tax_rate;

    // 3. Implement apply_discount closure here
    let apply_discount = |total_price: &mut f64, discount| {
        *total_price = *total_price - discount;
    };

    // 4. Implement checkout_cart closure here
    let checkout_cart = move |details: String| {
        let mut checkout: String = "Checkout complete: ".to_string();
        checkout.push_str(&details);
        checkout
    };

    (calculate_total, apply_discount, checkout_cart)
}

// Example usage
pub fn main() {
    let (calculate_total, mut apply_discount, checkout_cart) = create_typed_closures();

    // Example tests
    assert_eq!(calculate_total(100.0, 0.2), 120.0);

    let mut total_price = 120.0;
    apply_discount(&mut total_price, 20.0);
    assert_eq!(total_price, 100.0);

    let cart_details = String::from("Items: Apple, Banana, Orange");
    assert_eq!(
        checkout_cart(cart_details),
        "Checkout complete: Items: Apple, Banana, Orange"
    );
}
