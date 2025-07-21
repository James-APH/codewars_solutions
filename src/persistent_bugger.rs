fn persistence(num: u64) -> u64 {
    let mut temp_num = num;
    let mut count = 0;
    while temp_num >= 10 {
        temp_num = temp_num
            .to_string()
            .chars()
            .map(|x| x.to_digit(10).unwrap() as u64)
            .product();
        count += 1;
    }
    count
}
