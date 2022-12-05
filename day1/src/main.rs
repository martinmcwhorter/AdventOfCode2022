fn main() {
    let mut groups: Vec<u32> = vec![];

    include_str!("../input.txt")
            .lines()
            .collect::<Vec<&str>>()
            .split(|item| item.to_string() == "")
            .for_each(|lines| 
                groups.push(lines.into_iter()
                    .map(|n| match n.parse::<u32>() {
                        Ok(x) => x,
                        Err(_) => 0,
                    })
                    .reduce(|accum, item| accum + item)
                    .unwrap()
                )
            );          
            
    groups.sort();
    groups.reverse();
    let mut groups_iter = groups.into_iter();
    let result = groups_iter.nth(0).unwrap();

    println!("{}", result);
    
    let second = groups_iter.nth(0).unwrap();
    let third = groups_iter.nth(0).unwrap();
    let top_three = second + third + result;

    println!("Top three: {}", top_three);

}