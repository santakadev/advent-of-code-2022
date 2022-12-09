use std::{fs, str::FromStr, collections::HashSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Knot {
    position: Position,
    prev: Option<Box<Knot>>
}

impl Knot {
    fn new() -> Knot {
        Knot {
            position: Position {
                x: 0,
                y: 0,
            },
            prev: None,
        }
    }

    fn update_tail(&mut self, dir: &mut Direction) -> Position {
        if self.prev.is_none() {
            return self.position;
        }

        let prev = self.prev.as_mut().unwrap();

        let x_diff = self.position.x - prev.position.x;
        let y_diff = self.position.y - prev.position.y;

        // don't move
        if x_diff.abs() < 2 && y_diff.abs() < 2 {
            return prev.update_tail(dir);
        }

        // doble diagonal
        if x_diff.abs() == 2 && y_diff.abs() == 2 {
            prev.position.x += x_diff / 2;
            prev.position.y += y_diff / 2;
            return prev.update_tail(dir);
        }

        // update in same row
        if self.position.y == prev.position.y {
            prev.position.x += x_diff / 2;
            prev.position.y += y_diff / 2;
            return prev.update_tail(dir);
        }

        // update in same column
        if self.position.x == prev.position.x {
            prev.position.x += x_diff / 2;
            prev.position.y += y_diff / 2;
            return prev.update_tail(dir);
        }

        // update L-shape
        if x_diff > 0 && y_diff > 0 {
            prev.position.x += 1;
            prev.position.y += 1;
            return prev.update_tail(dir);
        } else if x_diff > 0 && y_diff < 0 {
            prev.position.x += 1;
            prev.position.y -= 1;
            return prev.update_tail(dir);
        } else if x_diff < 0 && y_diff > 0 {
            prev.position.x -= 1;
            prev.position.y += 1;
            return prev.update_tail(dir);
        } else if x_diff < 0 && y_diff < 0 {
            prev.position.x -= 1;
            prev.position.y -= 1;
            return prev.update_tail(dir);
        }
            
        return prev.update_tail(dir);
    }
}

struct Rope {
    head: Knot,
    visited: HashSet<Position>
}

impl Rope {
    fn new(length: usize) -> Rope {
        let mut head = Knot::new();
        let mut current = &mut head;
        let visited: HashSet<Position> = HashSet::new();

        for _ in 0..length {
            let prev = Knot::new();
            current.prev = Some(Box::new(prev));
            current = current.prev.as_mut().unwrap();
        }

        Rope {
            head,
            visited,
        }
    }

    fn move_rope(&mut self, dir: Direction) {
        let mut dir = dir;
        match dir {
            Direction::Up => self.head.position.y += 1,
            Direction::Down => self.head.position.y -= 1,
            Direction::Left => self.head.position.x -= 1,
            Direction::Right => self.head.position.x += 1,
        }

        let tail_position = self.head.update_tail(&mut dir);

        self.visited.insert(tail_position);
    }
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct Motion {
    direction: Direction,
    steps: u32,
}

impl FromStr for Motion {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (direction, steps) = s.split_once(" ").unwrap();
        let direction = match direction {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => panic!("Unknown direction"),
        };
        let steps: u32 = steps.parse().unwrap();
        Ok(Motion {
            direction,
            steps,
        })
    }

}

impl Motion {
    fn to_steps(&self) -> Vec<Step> {
        let mut steps = Vec::new();
        for _ in 0..self.steps {
            steps.push(Step {
                direction: self.direction,
            });
        }
        steps
    }
}

#[derive(Debug)]
struct Step {
    direction: Direction
}

fn main() {
    let input = fs::read_to_string("src/input.txt").expect("Error reading input.txt");

    let motions = input.trim_end().split("\n").map(|line| Motion::from_str(line).unwrap()).collect::<Vec<Motion>>();
    let steps = motions.iter().flat_map(|motion| motion.to_steps()).collect::<Vec<Step>>();

    let mut rope = Rope::new(9);

    for step in steps.iter() {
        rope.move_rope(step.direction);
    }

    println!("Visited count {:?}", rope.visited.iter().count());
}
