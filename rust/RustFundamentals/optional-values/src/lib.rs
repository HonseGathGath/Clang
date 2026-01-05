pub fn find_first_even(numbers: &[i32]) -> Option<i32> {
    let mut ret_val: Option<i32> = None;

    for nb in numbers.iter() {
        if *nb % 2 == 0 {
            ret_val = Some(*nb);
            break;
        }
    }

    ret_val
}

// Example usage
pub fn main() {
    let nums1 = vec![1, 3, 5, 8];
    let nums2 = vec![1, 3, 5];

    println!("{:?}", find_first_even(&nums1)); // Output: Some(8)
    println!("{:?}", find_first_even(&nums2)); // Output: None
}
