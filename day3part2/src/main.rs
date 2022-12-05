use std::collections::HashSet;

fn main() {
    let mut result = 0;

    include_str!("../input.txt")
        .lines()
        .collect::<Vec<_>>()
        .chunks(3)
        .for_each(|sacks| {

            println!("{:?}", sacks);

            let first = sacks.into_iter().nth(0).unwrap().chars().map(|x| x.clone()).collect::<HashSet<_>>();
            let second = sacks.into_iter().nth(0).unwrap().chars().map(|x| x.clone()).collect::<HashSet<_>>();
            let third = sacks.into_iter().nth(0).unwrap().chars().map(|x| x.clone()).collect::<HashSet<_>>();

            println!("{:?}", first);
            println!("{:?}", second);


            let first_two: HashSet<char> = first.intersection(&second).map(|x| x.clone()).collect();
            let all_three: HashSet<_> = first_two.intersection(&third).collect();

            println!("first two {:?}", first_two);
            println!("all_three {:?}", all_three);

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
