// My solution:
fn hello(name: &str) -> String {
    if let Some(head) = name.chars().next() {
        let temp_name = head.to_ascii_uppercase();
        let tail = &String::from(name)[1..name.len()].to_ascii_lowercase();
        String::from("Hello, ") + temp_name.to_string().as_str() + tail + "!"
    } else {
        String::from("Hello, World!")
    }
}

// Inspired Solution:
fn hello_new(name: &str) -> String {
    if name.is_empty() {
        String::from("Hello, World!")
    } else {
        let head = name[..1].to_ascii_uppercase();
        let tail = name[1..].to_ascii_lowercase();
        format!("Hello, {head}{tail}!")
    }
}
