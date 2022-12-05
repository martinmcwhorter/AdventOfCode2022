fn main() {

    let mut score = 0; 

    include_str!("../input.txt")
        .lines()
        .for_each(|line| {
            let you_s = line.chars().nth(0).unwrap().to_string();
            let you = you_s.as_str();
            let outcome_s = line.chars().nth(2).unwrap().to_string();
            let outcome = outcome_s.as_str();

            match outcome {
                "X" => score += 0,
                "Y" => score += 3,
                "Z" => score += 6,
                &_ => todo!()
            }

            match you {
                "A" => match outcome {
                    "X" => score += 3,
                    "Y" => score += 1,
                    "Z" => score += 2,
                    &_ => todo!()

                },
                "B" => match outcome {
                    "X" => score += 1,
                    "Y" => score += 2,
                    "Z" => score += 3,
                    &_ => todo!()

                },
                "C" => match outcome {
                    "X" => score += 2,
                    "Y" => score += 3,
                    "Z" => score += 1,
                    &_ => todo!()
                },
                &_ => todo!()
            }
        });

        println!{"{}", score};

}
