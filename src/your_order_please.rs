fn get_num_from_str(s: &str) -> u32 {
    for c in s.chars() {
        match c.is_ascii_digit() {
            true => return c.to_digit(10).unwrap(),
            _ => continue,
        };
    }
    0
}

pub fn order(sentence: &str) -> String {
    let mut split_sentence = sentence.split_ascii_whitespace().collect::<Vec<&str>>();
    split_sentence.sort_by_key(|x| get_num_from_str(x));
    split_sentence.join(" ")
}
