// use std::collections::HashMap;

use std::collections::HashSet;

fn main() {
    let mut result = 0;

    include_str!("../input.txt").lines().for_each(|sack| {
        let sack_vec = sack.chars().collect::<Vec<char>>();
        let (first, second) = sack_vec.split_at(sack.len() / 2);
        let first_set: HashSet<&char> = first.into_iter().collect();
        let second_set: HashSet<&char> = second.into_iter().collect();

        let in_both: HashSet<_> = first_set.intersection(&second_set).collect();

        in_both.iter().for_each(|item| 
            result += priority(***item)
        );
    });

    println!("{}", result);
}

fn priority(c: char) -> u32 {
    if c.is_uppercase() {
        return c as u32 - 38;
    }
    return c as u32 - 96;
}
