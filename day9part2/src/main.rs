use std::collections::{HashSet};

const NUMBER_OF_KNOTS: usize = 10;

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

struct Rope {
    visited: HashSet<Point>,
    knots: Vec<Point>,
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
            knots: vec![Point::new(0, 0); NUMBER_OF_KNOTS],
            moves: Self::process_input(input),
        } 
    }

    fn move_head(&mut self, m: Move) {
        match m {
            Move::Up => self.knots[0].up(),
            Move::Down => self.knots[0].down(),
            Move::Left => self.knots[0].left(),
            Move::Right => self.knots[0].right()
        }

    }

    fn distance(&self, knot: usize) -> (isize, isize) {
        return (self.knots[knot - 1].x - self.knots[knot].x, self.knots[knot - 1].y - self.knots[knot].y);
    }

    fn move_knot(&mut self, knot: usize) {
        let (x, y) = self.distance(knot);

        if x > 1 {
            self.knots[knot].right();
            if y >= 1 {
                self.knots[knot].up();
    
            }
            if y <= -1 {
                self.knots[knot].down();
            }
        } else 
        if x < -1 {
            self.knots[knot].left();
            if y >= 1 {
                self.knots[knot].up();
    
            }
            if y <= -1 {
                self.knots[knot].down();
            }
        } else
         
        if y > 1 {
            self.knots[knot].up();
            if x >= 1 {
                self.knots[knot].right();
            }
            if x <= -1 {
                self.knots[knot].left();
            }
        }  else 
        if y < -1 {
            self.knots[knot].down();
            if x >= 1 {
                self.knots[knot].right();
            }
            if x <= -1 {
                self.knots[knot].left();
            }
        }

        if knot == self.knots.len() - 1 {
            self.visited.insert(self.knots[knot].clone());
        }
    }

    fn simulate(&mut self) {
        for (m, distance) in self.moves.clone() {
            for _ in 0..distance {
                self.move_head(m.clone());
                for n in 1..(self.knots.len()) {
                    self.move_knot(n);
                }
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
        return Rope::from_input("R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20");
    }

    #[test]
    fn visited_count() {
        let mut sut = fixture();

        sut.simulate();
        let result = sut.position_count();

        assert_eq!(result, 36);
    }

    #[test] 
    fn visited() {
        let mut sut = fixture();

        sut.simulate();
        let result = sut.visited;

        assert!(result.contains(&Point::new(0, 0)));
        assert!(result.contains(&Point::new(1, 1)));
        assert!(result.contains(&Point::new(2, 2)));
        assert!(result.contains(&Point::new(1, 3)));
    }
}