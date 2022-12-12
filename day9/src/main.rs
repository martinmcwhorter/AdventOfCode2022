use std::collections::HashSet;

#[derive(Clone, Eq, Hash, PartialEq, Debug)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    fn up(&mut self) {
        self.y += 1;
    }

    fn down(&mut self) {
        self.y -= 1;
    }

    fn left(&mut self) {
        self.x -= 1;
    }
    
    fn right(&mut self) {
        self.x += 1;
    }
}

#[derive(Clone, Debug)]
enum Move {
    Left,
    Right,
    Up,
    Down,
}

#[derive()]
struct Rope {
    visited: HashSet<Point>,
    head: Point,
    tail: Point,
    moves: Vec<(Move, u8)>,
}

impl Rope {
    fn process_input(input: &str) -> Vec<(Move, u8)> {
        return input
            .lines()
            .map(|l| {
                let (command_str, distance_str) = l.split_once(" ").unwrap();
                let distance: u8 = distance_str.parse().unwrap();
                match command_str {
                    "L" => return (Move::Left, distance),
                    "R" => return (Move::Right, distance),
                    "U" => return (Move::Up, distance),
                    "D" => return (Move::Down, distance),
                    &_ => panic!("Unknown command string {}", command_str)
                }
            })
            .collect();
    }

    fn from_input(input: &str) -> Self {
        Self {
            visited: HashSet::new(),
            head: Point::new(0, 0),
            tail: Point::new(0, 0),
            moves: Self::process_input(input),
        } 
    }

    fn move_head(&mut self, m: Move) {
        match m {
            Move::Up => self.head.up(),
            Move::Down => self.head.down(),
            Move::Left => self.head.left(),
            Move::Right => self.head.right()
        }

    }

    fn distance(&self) -> (isize, isize) {
        return (self.head.x - self.tail.x, self.head.y - self.tail.y);
    }

    fn move_tail(&mut self) {
        let (x, y) = self.distance();
        if x > 1 {
            self.tail.right();
            self.tail.y = self.head.y;
        }
        if x < -1 {
            self.tail.left();
            self.tail.y = self.head.y;
        }
        if y > 1 {
            self.tail.up();
            self.tail.x = self.head.x;
        }
        if y < -1 {
            self.tail.down();
            self.tail.x = self.head.x;
        }

        self.visited.insert(self.tail.clone());
    }
    fn simulate(&mut self) {
        for (m, distance) in self.moves.clone() {
            for _ in 0..distance {
                self.move_head(m.clone());
                self.move_tail();
            }
        }
    }

    fn position_count(&self) -> usize {
        return self.visited.len();
    }

}

fn main() {
    let mut rope = Rope::from_input(include_str!("../input.txt"));
    rope.simulate();
    println!("{}", rope.position_count());
}

#[cfg(test)]
mod test {
    use crate::{Rope, Point};

    fn fixture() -> Rope {
        return Rope::from_input("R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2");
    }

    #[test]
    fn visited_count() {
        let mut sut = fixture();

        sut.simulate();
        let result = sut.position_count();

        assert_eq!(result, 13);
    }

    #[test] 
    fn visited() {
        let mut sut = fixture();

        sut.simulate();
        let result = sut.visited;

        assert!(result.contains(&Point::new(0, 0)));
        assert!(result.contains(&Point::new(1, 0)));
        assert!(result.contains(&Point::new(2, 0)));
        assert!(result.contains(&Point::new(2, 0)));
        assert!(result.contains(&Point::new(4, 1)));
        assert!(result.contains(&Point::new(1, 2)));
        assert!(result.contains(&Point::new(2, 2)));
        assert!(result.contains(&Point::new(3, 2)));
        assert!(result.contains(&Point::new(4, 2)));
        assert!(result.contains(&Point::new(3, 3)));
        assert!(result.contains(&Point::new(4, 3)));
        assert!(result.contains(&Point::new(2, 4)));
        assert!(result.contains(&Point::new(3, 4)));
    }
}