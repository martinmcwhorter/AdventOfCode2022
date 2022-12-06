use std::collections::HashMap;

fn main() {
    let (stacks, instructions) = include_str!("../input.txt").split_once("\n\n").unwrap();

    let stack_keys: Vec<u8> = stacks
        .lines()
        .rev()
        .nth(0)
        .unwrap()
        .split(" ")
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<u8>().unwrap())
        .collect();

    let mut stack_map: HashMap<u8, Vec<char>> = stack_keys.clone().into_iter().map(|k| (k, Vec::new())).collect();

    stacks.lines().rev().skip(1).for_each(|line| {
        stack_keys.clone().into_iter().for_each(|key| {
            let p = position(key);
            let container = line.chars().nth(p.into()).unwrap();

            if container == ' ' {
                return;
            }

            stack_map.entry(key).and_modify(|f| f.push(container));
        })
    });

    instructions
        .lines()
        .map(|line| line.split_whitespace())
        .for_each(|inst| {
            let amount: u8 = inst.clone().nth(1).unwrap().parse().unwrap();
            let from: u8 = inst.clone().nth(3).unwrap().parse().unwrap();
            let to: u8 = inst.clone().nth(5).unwrap().parse().unwrap();

            for _a in 0..amount {
                match stack_map.get_mut(&from).unwrap().pop() {
                    Some(c) => stack_map.get_mut(&to).unwrap().push(c),
                    None => todo!()
                }
            }
        });

    stack_keys.into_iter().for_each(|key| print!("{}", stack_map.get_mut(&key).unwrap().pop().unwrap_or_default()));
    println!();

}

fn position(k: u8) -> u8 {
    return (k - 1) * 4 + 1;
}
