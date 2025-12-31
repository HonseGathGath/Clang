pub fn update_slice(slice: &mut [i32], indices: &[usize], value: i32) {
    for indice in indices {
        for (idx, nb) in slice.iter_mut().enumerate() {
            if idx == *indice {
                *nb = value;
            }
        }
    }
}
