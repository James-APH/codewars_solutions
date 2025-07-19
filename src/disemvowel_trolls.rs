/// First attempt before googling and learning about replace.
/*
fn disemvowel(s: &str) -> String {
    s.chars()
        .filter(|x| !['a', 'e', 'i', 'o', 'u'].contains(&x.to_lowercase().next().unwrap()))
        .collect::<String>()
}
*/
fn disemvowel(s: &str) -> String {
    s.replace(['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'], "")
}
