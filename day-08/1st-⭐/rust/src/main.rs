use std::fs;
use std::cmp::max;

struct Grid {
    size: usize,
    trees: Vec<Tree>
}

impl Grid {
    fn get_tree_index(&self, x: usize, y: usize) -> usize {
        y * self.size + x
    }
}

#[derive(Debug)]
struct Tree {
    height: u32,
    position: (usize, usize),
    visible: bool
}

impl Tree {
    fn new(height: u32, position: (usize, usize)) -> Tree {
        Tree {
            height,
            position,
            visible: false
        }
    }

    fn is_visible(&self) -> bool {
        self.visible
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

    let mut grid = Grid {
        size,
        trees
    };

    let mut left_max_seen = 0;
    let mut right_max_seen = 0;
    let mut top_max_seen = 0;
    let mut bottom_max_seen = 0;
    for i in 0..grid.trees.len() {
        left_max_seen = check_left(&mut grid, i, left_max_seen);
        right_max_seen = check_right(&mut grid, i, right_max_seen);
        top_max_seen = check_top(&mut grid, i, top_max_seen);
        bottom_max_seen = check_bottom(&mut grid, i, bottom_max_seen);
    }

    let visible_trees = grid.trees.iter().filter(|tree| tree.is_visible()).count();

    println!("{:?}", visible_trees);
}

fn check_left(grid: &mut Grid, i: usize, max_seen: u32) -> u32 {
    let x = i % grid.size;
    let y = i / grid.size;
    return check(grid, x, y, max_seen);
}

fn check_right(grid: &mut Grid, i: usize, max_seen: u32) -> u32 {
    let x = (grid.size - 1) - (i % grid.size);
    let y = i / grid.size;
    return check(grid, x, y, max_seen);
}

fn check_top(grid: &mut Grid, i: usize, max_seen: u32) -> u32 {
    let x = i / grid.size;
    let y = i % grid.size;
    return check(grid, x, y, max_seen);
}

fn check_bottom(grid: &mut Grid, i: usize, max_seen: u32) -> u32 {
    let x = i / grid.size;
    let y = (grid.size - 1) - (i % grid.size);
    return check(grid, x, y, max_seen);
}

fn check(grid: &mut Grid, x: usize, y: usize, max_seen: u32) -> u32 {
    let index = grid.get_tree_index(x, y);
    let current_tree = &mut grid.trees[index];
    let x = current_tree.position.0;
    let y = current_tree.position.1;

    if x == 0 || y == 0 || x == grid.size - 1 || y == grid.size - 1 {
        current_tree.visible = true;
        return current_tree.height;
    }

    if current_tree.height > max_seen {
        current_tree.visible = true;
        return max(current_tree.height, max_seen);
    }

    return max_seen;
}
