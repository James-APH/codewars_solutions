/// First Solution
/*
    fn solution(num: i32) -> i32 {
        if num <= 0 {
            0
        } else {
            (0..num)
                .collect::<Vec<i32>>()
                .into_iter()
                .filter(|x| x % 3 == 0 || x % 5 == 0)
                .reduce(|acc, e| acc + e)
                .unwrap()
        }
    }
*/
/// Second solution, did not realize there was a sum func

fn solution(num: i32) -> i32 {
    (0..num).filter(|x| x % 3 == 0 || x % 5 == 0).sum()
}
