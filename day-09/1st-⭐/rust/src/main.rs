use std::{fs, str::FromStr, collections::HashSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: i32,
    y: i32,
}

struct Head {
    position: Position
}

impl Head {
    fn new() -> Head {
        Head {
            position: Position {
                x: 0,
                y: 0,
            }
        }
    }
}

struct Tail {
    position: Position
}

impl Tail {
    fn new() -> Tail {
        Tail {
            position: Position {
                x: 0,
                y: 0,
            }
        }
    }

    fn update(&mut self, head: &Head, head_dir: Direction) {
        // don't move
        let x_diff = (head.position.x - self.position.x).abs();
        let y_diff = (head.position.y - self.position.y).abs();

        if x_diff < 2 && y_diff < 2 {
            return;
        }


        // update in same row
        if self.position.y == head.position.y {
            match head_dir {
                Direction::Right => self.position.x += 1,
                Direction::Left => self.position.x -= 1,
                _ => {}
            }
        }

        // update in same column
        else if self.position.x == head.position.x {
            match head_dir {
                Direction::Up => self.position.y += 1,
                Direction::Down => self.position.y -= 1,
                _ => {}
            }
        }
        
        else {
            // update in diagonal
            match head_dir {
                Direction::Right =>  {
                    self.position.x += 1;
                    self.position.y = head.position.y;
                }
                Direction::Left => {
                    self.position.x -= 1;
                    self.position.y = head.position.y;
                }
                Direction::Up => {
                    self.position.y += 1;
                    self.position.x = head.position.x;
                }
                Direction::Down => {
                    self.position.y -= 1;
                    self.position.x = head.position.x;
                }
            }
        }
    }
}

struct Rope {
    head: Head,
    tail: Tail,
}

impl Rope {
    fn new() -> Rope {
        Rope {
            head: Head::new(),
            tail: Tail::new(),
        }
    }

    fn move_rope(&mut self, dir: Direction) {
        match dir {
            Direction::Up => self.head.position.y += 1,
            Direction::Down => self.head.position.y -= 1,
            Direction::Left => self.head.position.x -= 1,
            Direction::Right => self.head.position.x += 1,
        }

        self.tail.update(&self.head, dir);
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

    let mut rope = Rope::new();
    let mut visited: HashSet<Position> = HashSet::new();

    for step in steps {
        rope.move_rope(step.direction);
        visited.insert(rope.tail.position);
    }

    println!("Visited count {:?}", visited.iter().count());
}
