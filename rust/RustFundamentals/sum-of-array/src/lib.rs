pub fn sum_array(arr: &[i32]) -> i32 {
    let mut sum: i32 = 0;
    for nb in arr.iter() {
        sum += nb;
    }
    sum
}
