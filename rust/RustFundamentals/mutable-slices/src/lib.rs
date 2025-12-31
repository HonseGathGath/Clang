pub fn transform_even_odd(slice: &mut [i32]) {
    for n in slice {
        if *n % 2 == 0 {
            *n = 2 * *n;
        } else {
            *n = *n - 1;
        }
    }
}
