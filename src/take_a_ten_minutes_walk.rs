fn is_valid_walk(walk: &[char]) -> bool {
    if walk.len() > 10 || walk.len() < 10 {
        false
    } else {
        let mut compass = (0, 0);
        for dir in walk {
            match dir {
                'n' => compass.0 += 1,
                'w' => compass.1 += 1,
                's' => compass.0 -= 1,
                'e' => compass.1 -= 1,
                _ => {}
            }
        }
        compass.0 == 0 && compass.1 == 0
    }
}
