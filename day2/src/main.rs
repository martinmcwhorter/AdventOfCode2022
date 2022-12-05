fn main() {

    let mut score = 0; 

    include_str!("../input.txt")
        .lines()
        .for_each(|line| {
            let you_s = line.chars().nth(0).unwrap().to_string();
            let you = you_s.as_str();
            let me_s = line.chars().nth(2).unwrap().to_string();
            let me = me_s.as_str();

            match me {
                "X" => score += 1,
                "Y" => score += 2,
                "Z" => score += 3,
                &_ => todo!()
            }

            match you {
                "A" => match me {
                    "X" => score += 3,
                    "Y" => score += 6,
                    "Z" => score += 0,
                    &_ => todo!()

                },
                "B" => match me {
                    "X" => score += 0,
                    "Y" => score += 3,
                    "Z" => score += 6,
                    &_ => todo!()

                },
                "C" => match me {
                    "X" => score += 6,
                    "Y" => score += 0,
                    "Z" => score += 3,
                    &_ => todo!()
                },
                &_ => todo!()
            }
        });

        println!{"{}", score};

}
