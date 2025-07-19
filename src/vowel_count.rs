/// definitly not the simplest way to do this problem,
/// but I am trying to use some of these to learn various
/// rust functions, and get used to using closures

/*
fn get_count(string: &str) -> usize {
    string
        .chars()
        .filter(|x| ['a', 'e', 'i', 'o', 'u'].contains(x))
        .collect::<Vec<char>>()
        .len()
}
*/

/// loooking at other peoples solutions, I can simplify mine a little:
fn get_count(string: &str) -> usize {
    string
        .chars()
        .filter(|x| ['a', 'e', 'i', 'o', 'u'].contains(x))
        .count() // removing collect and len which was accomplishing the samething
}
