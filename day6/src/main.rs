use std::collections::HashSet;

fn main() {
    let stream = include_str!("../input.txt").to_string();

    println!("start of packet {}", find_start_of_segment(4, &stream));
    println!("start of message {}", find_start_of_segment(14, &stream));
}

fn find_start_of_segment(size: usize, stream: &String) -> usize {
    let mut i = 0;

    loop {
        let slice = &stream[i..i + size];
        let slice_set: HashSet<char>= slice.chars().collect(); 

        if slice_set.len() == size {
            return i + size;
        }

        i += 1;
    }
}