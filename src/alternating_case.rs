use itertools::Itertools;

// my way of doing it
fn to_alternating_case(s: &str) -> String {
    s.chars()
        .map(|x: char| {
            if x.is_lowercase() {
                x.to_ascii_uppercase()
            } else if x.is_uppercase() {
                x.to_ascii_lowercase()
            } else {
                x
            }
        })
        .join("")
        .to_string()
}

// Inspired solution, after looking at others:
fn to_alternating_case_two(s: &str) -> String {
    s.chars()
        .map(|x: char| match x {
            x if x.is_lowercase() => x.to_ascii_uppercase(),
            x if x.is_uppercase() => x.to_ascii_lowercase(),
            _ => x,
        })
        .collect::<String>()
}
