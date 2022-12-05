use std::collections::HashSet;

fn main() {
    let mut result = 0;

    include_str!("../input.txt")
        .lines()
        .collect::<Vec<_>>()
        .chunks(3)
        .for_each(|sacks| {

            let mut sacks_iter = sacks.into_iter();
            let first: HashSet<_> = sacks_iter.nth(0).unwrap().chars().map(|x| x.clone()).collect();
            let second: HashSet<_> = sacks_iter.nth(0).unwrap().chars().map(|x| x.clone()).collect();
            let third: HashSet<_> = sacks_iter.nth(0).unwrap().chars().map(|x| x.clone()).collect();

            let first_two: HashSet<char> = first.intersection(&second).map(|x| x.clone()).collect();
            let all_three: HashSet<_> = first_two.intersection(&third).collect();

            all_three.iter().for_each(|item| result += priority(**item));
        });

    println!("{}", result);
}

fn priority(c: char) -> u32 {
    if c.is_uppercase() {
        return c as u32 - 38;
    }
    return c as u32 - 96;
}
