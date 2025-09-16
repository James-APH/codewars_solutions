use itertools::enumerate;

// My way of doing it:
fn remove_every_other(arr: &[u8]) -> Vec<u8> {
    let mut filter_vec = vec![];
    for (index, element) in enumerate(arr) {
        if index % 2 == 0 {
            filter_vec.push(*element);
        }
    }
    filter_vec
}

// better way, learned from other solutions:
fn better_remove_every_other(arr: &[u8]) -> Vec<u8> {
    arr.iter().step_by(2).copied().collect()
}
