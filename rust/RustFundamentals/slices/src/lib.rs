pub fn find_largest_in_slice(slice: &[i32]) -> Option<i32> {
    let max: Option<i32> = {
        if slice.iter().count() == 0 {
            None
        } else {
            let mut temp_max: i32 = slice.get(0).copied().unwrap_or(0);
            for n in slice {
                if temp_max < *n {
                    temp_max = *n;
                }
            }
            Some(temp_max)
        }
    };
    max
}

// Example Usage
pub fn main() {
    let numbers = [1, 3, 7, 2, 5];
    assert_eq!(find_largest_in_slice(&numbers), Some(7));

    let empty: [i32; 0] = [];
    assert_eq!(find_largest_in_slice(&empty), None);

    let single_element = [42];
    assert_eq!(find_largest_in_slice(&single_element), Some(42));
}
