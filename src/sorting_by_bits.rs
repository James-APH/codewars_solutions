use std::cmp::Ordering;

// My Initial Solution:
/*
fn is_less(n1: &u32, n2: &u32) -> Ordering {
    let sum1 = format!("{n1:b}").matches('1').count();
    let sum2 = format!("{n2:b}").matches('1').count();
    if sum1 == sum2 {
        return n1.cmp(n2);
    }
    sum1.cmp(&sum2)
}
*/

// Solution after looking at others code:
fn is_less(n1: &u32, n2: &u32) -> Ordering {
    let sum1 = n1.count_ones();
    let sum2 = n2.count_ones();
    if sum1 == sum2 {
        return n1.cmp(n2);
    }
    sum1.cmp(&sum2)
}

fn sort_by_bit(arr: &mut Vec<u32>) {
    arr.sort_by(is_less);
}
