fn main() {
    let parsed: Vec<Vec<Vec<u32>>> = include_str!("../input.txt")
        .lines()
        .map(|line| {
            line.split(",")
                .map(|range| {
                    range
                        .split("-")
                        .map(|n| n.parse::<u32>().unwrap())
                        .collect()
                })
                .collect()
        })
        .collect();

    let result: u32 = parsed.clone().into_iter().fold(0, |accum, pair| {
            let first = pair.first().unwrap();
            let second = pair.last().unwrap();
            
            let first_min = first.first().unwrap();
            let first_max = first.last().unwrap();
            let second_min = second.first().unwrap();
            let second_max = second.last().unwrap();

            if first_min >= second_min && first_max <= second_max {
                return accum + 1;
            } else if second_min >= first_min && second_max <= first_max {
                return accum +1
            } 
            return accum;
        });

    println!("{}", result);

    let part2 = parsed.into_iter().fold(0, |accum, pair| {
        let first = pair.first().unwrap();
        let second = pair.last().unwrap();
        
        let first_min = first.first().unwrap();
        let first_max = first.last().unwrap();
        let second_min = second.first().unwrap();
        let second_max = second.last().unwrap();

        if first_min >= second_min && first_min <= second_max {
            return accum + 1;
        } else if second_min >= first_min && second_min <= first_max {
            return accum +1
        } 
        return accum;
    });

    println!("part two {}", part2);

}
