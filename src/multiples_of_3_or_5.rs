pub fn solution(num: i32) -> i32 {
    (0..num)
        .collect::<Vec<i32>>()
        .into_iter()
        .filter(|x| x % 3 == 0 || x % 5 == 0)
        .reduce(|acc, e| acc + e)
        .unwrap()
}
