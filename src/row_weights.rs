fn row_weights(array: Vec<u32>) -> (u32, u32) {
    let mut t0 = 0;
    let mut t1 = 0;
    for (index, ele) in array.iter().enumerate() {
        match index % 2 {
            0 => t0 += ele,
            _ => t1 += ele,
        }
    }
    (t0, t1)
}
