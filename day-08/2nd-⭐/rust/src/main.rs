use std::fs;

#[derive(PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Grid {
    size: usize,
    trees: Vec<Tree>
}

impl Grid {
    fn get_tree(&self, position: (usize, usize)) -> &Tree {
        return &self.trees[self.get_tree_index(position.0, position.1)];
    }
    
    fn get_tree_index(&self, x: usize, y: usize) -> usize {
        y * self.size + x
    }

    fn to_scores(&self) -> Vec<Score> {
        self.trees.iter().map(|tree| self.score_for(tree.position)).collect()
    }

    fn score_for(&self, position: (usize, usize)) -> Score {
        let mut score = Score::new();
        let target_tree_height = self.get_tree(position).height;
        self.walk(position, None, target_tree_height, &mut score);
        score
    }

    fn walk(&self, cur_pos: (usize, usize), dir: Option<Direction>, height: u32, score: &mut Score) {
        let is_target = dir.is_none();

        let cur = self.get_tree(cur_pos);

        if !is_target && cur.height >= height {
            return;
        }

        // recurse left
        if (is_target || dir == Some(Direction::Left)) && cur_pos.0 > 0 {
            let next_x = cur_pos.0 - 1;
            let next_y = cur_pos.1;
            self.walk((next_x, next_y), Some(Direction::Left), height, score);
            score.left += 1;
        }

        // recurse right
        if (is_target || dir == Some(Direction::Right)) && cur_pos.0 < self.size - 1 {
            let next_x = cur_pos.0 + 1;
            let next_y = cur_pos.1;
            self.walk((next_x, next_y), Some(Direction::Right), height, score);
            score.right += 1;
        }

        // recurse up
        if (is_target || dir == Some(Direction::Up)) && cur_pos.1 > 0 {
            let next_x = cur_pos.0;
            let next_y = cur_pos.1 - 1;
            self.walk((next_x, next_y), Some(Direction::Up), height, score);
            score.up += 1;
        }
        // recurse bottom
        if (is_target || dir == Some(Direction::Down)) && cur_pos.1 < self.size - 1 {
            let next_x = cur_pos.0;
            let next_y = cur_pos.1 + 1;
            self.walk((next_x, next_y), Some(Direction::Down), height, score);
            score.down += 1;
        }
    }
}

#[derive(Debug)]
struct Score {
    left: usize,
    right: usize,
    up: usize,
    down: usize
}

impl Score {
    fn new() -> Score {
        Score {
            left: 0,
            right: 0,
            up: 0,
            down: 0
        }
    }

    fn scenic(&self) -> u32 {
        self.left as u32 * self.right as u32 * self.up as u32 * self.down as u32
    }
}

#[derive(Debug)]
struct Tree {
    height: u32,
    position: (usize, usize)
}

impl Tree {
    fn new(height: u32, position: (usize, usize)) -> Tree {
        Tree {
            height,
            position,
        }
    }
}

fn main() {
    let input = fs::read_to_string("src/input.txt").expect("Unable to read file");

    let size = input.trim_end().lines().count();
    let trees_height = input.lines().flat_map(|line| line.chars()).map(|c| c.to_digit(10).unwrap()).collect::<Vec<u32>>();

    let trees = trees_height.iter().enumerate().map(|(i, height)| {
        let tree = Tree::new(*height, (i % size, i / size));
        tree
    }).collect::<Vec<Tree>>();

    let grid = Grid { size, trees };

    let max_scenic_score = grid.to_scores().iter().map(|score| score.scenic()).max().unwrap();
    println!("{:?}", max_scenic_score);
}
