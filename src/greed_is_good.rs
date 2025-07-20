use std::collections::{HashMap, HashSet};

fn get_count(dice: &[u8; 5], num: u8) -> u8 {
    dice.iter().filter(|&&x| x == num).count() as u8
}

fn score(dice: [u8; 5]) -> u32 {
    let three_roles = HashMap::from([(1, 1000), (6, 600), (5, 500), (4, 400), (3, 300), (2, 200)]);
    let one_role = HashMap::from([(1, 100), (5, 50)]);
    let mut dice_count: HashMap<u8, u8> = HashMap::new();
    let mut sum: u32 = 0;

    for role in HashSet::from(dice) {
        dice_count.insert(role, get_count(&dice, role));
    }

    for (role, count) in dice_count {
        let mut temp_count = count as u32;
        if count >= 3 {
            sum += three_roles[&role];
            temp_count -= 3;
        }

        if role == 1 || role == 5 {
            sum += temp_count * one_role[&role];
        }
    }
    sum
}
