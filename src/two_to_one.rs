/*
* use std::collections::HashSet;
// first attempt
fn longest(a1: &str, a2: &str) -> String {
    let unique_chars: HashSet<char> = format!("{a1}{a2}").chars().collect();
    let mut chars: Vec<char> = Vec::from_iter(unique_chars);
    chars.sort();
    chars.iter().collect()
}
*/
// modified first attempt after looking at other solutions, just wanted to remove the use of
// collections.
//
fn longest(a1: &str, a2: &str) -> String {
    let mut chars: Vec<char> = format!("{a1}{a2}").chars().collect();
    chars.sort();
    chars.dedup();
    chars.iter().collect()
}
