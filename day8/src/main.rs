#[derive(Debug)]
struct Position {
    x: usize,
    y: usize
}

impl Position {
    fn new(x: usize, y: usize) -> Self { Self { x, y } }
}

struct Forrest {
    trees: Vec<Vec<u8>>,
    max_x: usize,
    max_y: usize
}

impl Forrest {
    fn from_str(forrest: &str) -> Self { 
        let max_y = forrest.lines().count();
        let max_x = forrest.lines().nth(0).unwrap().len();
        let trees = forrest.lines().map(|l| l.chars().map(|c| c.to_string().parse().unwrap()).collect()).collect();

        Self { trees, max_x, max_y } 
    }

    fn tree_height(&self, p: &Position) -> u8 {
        return self.trees[p.y][p.x];
    }

    fn can_tree_be_seen(&self, p: &Position) -> (bool, u32) {   
        if p.x == 0 || p.x == self.max_x - 1 || p.y == 0 || p.y == self.max_y - 1 {
            return (true, 0);
        }

        let mut result = false;
        let tree_height = self.tree_height(p);
        
        const DISTANCE_MULTIPLIER: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

        let mut scenic_scores : Vec<u32> = vec![];

        DISTANCE_MULTIPLIER.into_iter().for_each(|m| {
            let mut distance = 1;

            loop {
                let x = (p.x as isize + m.0 * distance) as usize;
                let y = (p.y as isize + m.1 * distance) as usize;

                let test_point = Position::new(x, y);
                let test_height = self.tree_height(&test_point);
                if test_height >= tree_height {
                    break;
                } else if x >= self.max_x - 1 || x <= 0 || y >= self.max_y - 1|| y <= 0 {
                    result = true;
                    break;
                }

                distance += 1;
            }
            
            scenic_scores.push(distance as u32);
        });
    
        return (result, scenic_scores.into_iter().fold(1, |acc, s| acc * s));
    }
    
    fn visible_count(&self) -> (u32, u32) {
        let mut visible = 0;
        let mut scenic_scores: Vec<u32> = vec![];
        for y in 0..self.max_y {
            for x in 0..self.max_x {
                let (can_be_seen, scenic_score) = self.can_tree_be_seen(&Position::new(x, y));
                scenic_scores.push(scenic_score);
                if can_be_seen {
                    visible += 1;
                }
            }
        }

        scenic_scores.sort();
        return (visible, *scenic_scores.last().unwrap());
    }
}

fn main() {
    let forrest = Forrest::from_str(include_str!("../input.txt"));
    let (visible, scenic_score) = forrest.visible_count();
    println!("Visible Trees: {}", visible);
    println!("Scenic Score: {}", scenic_score);
}

#[cfg(test)]
mod tests {
    use crate::{Forrest, Position};


    fn fixture() -> &'static str  {
        return r#"30373
25512
65332
33549
35390"#
    }

    #[test]
    fn visible_count() {
        let forrest = Forrest::from_str(fixture());

        let result = forrest.visible_count();

        assert_eq!(21, result.0);
        assert_eq!(8, result.1);
    }

    #[test]
    fn tree_height() {
        let forrest = Forrest::from_str(fixture());

        assert_eq!(forrest.tree_height(&Position::new(0, 0)), 3);
        assert_eq!(forrest.tree_height(&Position::new(0, 4)), 3);
        assert_eq!(forrest.tree_height(&Position::new(4, 0)), 3);
        assert_eq!(forrest.tree_height(&Position::new(4, 4)), 0);
        assert_eq!(forrest.tree_height(&Position::new(3, 1)), 1);

    }

    #[test]
    fn can_tree_be_seen() {
        let forrest = Forrest::from_str(fixture());

        assert_eq!(forrest.can_tree_be_seen(&Position::new(1, 1)).0, true);
        assert_eq!(forrest.can_tree_be_seen(&Position::new(2, 1)).0, true);
        assert_eq!(forrest.can_tree_be_seen(&Position::new(3, 1)).0, false);

        assert_eq!(forrest.can_tree_be_seen(&Position::new(1, 2)).0, true);
        assert_eq!(forrest.can_tree_be_seen(&Position::new(2, 2)).0, false);
        assert_eq!(forrest.can_tree_be_seen(&Position::new(3, 2)).0, true);
    }

    #[test]
    fn can_tree_be_seen_scenic_score() {
        let forrest = Forrest::from_str(fixture());

        assert_eq!(forrest.can_tree_be_seen(&Position::new(2, 1)).1, 4);
        assert_eq!(forrest.can_tree_be_seen(&Position::new(2, 3)).1, 8);
    }
}