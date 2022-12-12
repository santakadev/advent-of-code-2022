use std::fs;

#[derive(Debug, Clone, PartialEq)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn new(x: usize, y: usize) -> Position {
        Position { x, y }
    }
}

struct Graph {
    width: usize,
    height: usize,
    positions: Vec<Vec<Option<Position>>>,
    edges: Vec<Vec<bool>>
}

impl Graph {
    fn new(width: usize, height: usize) -> Graph {
        Graph {
            width,
            height,
            positions: vec![vec![None; width]; height],
            edges: vec![vec![false; width * height]; width * height],
        }
    }

    fn add(&mut self, position: Position) {
        let x = position.x;
        let y = position.y;
        self.positions[y][x] = Some(position);
    }

    fn connect(&mut self, from: Position, to: Position) {
        let from_index = from.x + from.y * self.width;
        let to_index = to.x + to.y * self.width;
        self.edges[from_index][to_index] = true;
    }
    
    fn is_connected(&self, from: Position, to: Position) -> bool {
        let from_index = from.x + from.y * self.width;
        let to_index = to.x + to.y * self.width;
        self.edges[from_index][to_index]
    }

    fn shortest_path(&self, from: Position, to: Position) -> Option<usize> {
        let mut queue = Vec::new();
        let mut visited = vec![vec![false; self.width]; self.height];
        let mut distances = vec![vec![None; self.width]; self.height];
        queue.push(from.clone());
        visited[from.y][from.x] = true;
        distances[from.y][from.x] = Some(0);
        while !queue.is_empty() {
            let current = queue.remove(0);
            let current_distance = distances[current.y][current.x].unwrap();
            let neighbors = self.neighbors(current);
            for neighbor in neighbors {
                if !visited[neighbor.y][neighbor.x] {
                    visited[neighbor.y][neighbor.x] = true;
                    distances[neighbor.y][neighbor.x] = Some(current_distance + 1);
                    queue.push(neighbor.clone());
                }
            }
        }
        distances[to.y][to.x]
    }

    fn neighbors(&self, position: Position) -> Vec<Position> {
        let mut neighbors = Vec::new();
        let x = position.x;
        let y = position.y;
        if x > 0 && self.is_connected(position.clone(), Position::new(x - 1, y)) {
            neighbors.push(Position::new(x - 1, y));
        }
        if x < self.width - 1 && self.is_connected(position.clone(), Position::new(x + 1, y)) {
            neighbors.push(Position::new(x + 1, y));
        }
        if y > 0 && self.is_connected(position.clone(), Position::new(x, y - 1)) {
            neighbors.push(Position::new(x, y - 1));
        }
        if y < self.height - 1 && self.is_connected(position.clone(), Position::new(x, y + 1)) {
            neighbors.push(Position::new(x, y + 1));
        }
        neighbors
    }
}

fn main() {
    let input = fs::read_to_string("src/input.txt").unwrap();

    let mut map = input.
        trim_end()
        .split("\n")
        .map(|line| line.chars().map(|c| c as usize).collect::<Vec<usize>>())
        .collect::<Vec<_>>();

    let height = map.len();
    let width = map[0].len();

    let mut start = Position::new(0, 0);
    let mut end = Position::new(0, 0);

    let mut graph = Graph::new(width, height);

    for y in 0..height {
        for x in 0..width {
            if map[y][x] == 83 {
                start = Position::new(x, y);
                map[y][x] = 'a' as usize;
            } else if map[y][x] == 69 {
                end = Position::new(x, y);
                map[y][x] = 'z' as usize;
            }
        }
    }
    
    for y in 0..height {
        for x in 0..width {
            graph.add(Position::new(x, y));

            if x > 0 {
                if map[y][x - 1] as isize - map[y][x] as isize <= 1 {
                    graph.connect(Position::new(x, y), Position::new(x - 1, y));
                }
            }

            if x < width - 1 {
                if map[y][x + 1] as isize - map[y][x] as isize <= 1 {
                    graph.connect(Position::new(x, y), Position::new(x + 1, y));
                }
            }

            if y > 0 {
                if map[y - 1][x] as isize - map[y][x] as isize <= 1 {
                    graph.connect(Position::new(x, y), Position::new(x, y - 1));
                }
            }

            if y < height - 1 {
                if map[y + 1][x] as isize - map[y][x] as isize <= 1 {
                    graph.connect(Position::new(x, y), Position::new(x, y + 1));
                }
            }
        }
    }

    let shortest = graph.shortest_path(start, end).unwrap();
    println!("Shortest distance: {}", shortest);
}

