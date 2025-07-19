fn disemvowel(s: &str) -> String {
    s.chars()
        .filter(|x| !['a', 'e', 'i', 'o', 'u'].contains(x.to_lowercase()))
        .collect::<String>()
}

fn get_count(string: &str) -> usize {
    string
        .chars()
        .filter(|x| ['a', 'e', 'i', 'o', 'u'].contains(x))
        .count() // removing collect and len which was accomplishing the samething
}
