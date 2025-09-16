// probably not the most elegant solution, but will likely result in fewest assembly instructions
//fn digitize(n: u64) -> Vec<u8> {
//    n.to_string().split("").rev()
//    let mut t_n = n;
//    let mut digits = vec![];
//    while n > 0 {
//        digits.push((t_n % 10) as u8);
//        t_n /= 10;
//    }
//    digits
//}

fn digitize(n: u64) -> Vec<u8> {
    n.to_string()
        .chars()
        .rev()
        .map(|e| e.to_string().parse::<u8>().unwrap())
        .collect()
}
