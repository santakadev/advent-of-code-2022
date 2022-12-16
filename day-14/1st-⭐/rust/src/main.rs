use std::{fs, str::FromStr};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    Rock,
    Sand,
}

#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Path {
    points: Vec<Point>,
}

#[derive(Debug)]
struct Cave {
    tiles: Vec<Vec<Tile>>,
    lower_bound: usize,
}

impl FromStr for Point {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(',');
        let x = parts.next().unwrap().parse().unwrap();
        let y = parts.next().unwrap().parse().unwrap();
        Ok(Point { x, y })
    }
}

impl FromStr for Path {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s
            .split(" -> ")
            .map(|s| s.parse().unwrap())
            .collect::<Vec<Point>>();
        Ok(Path { points: lines })
    }

}

impl Cave {
    fn new() -> Self {
        return Cave { 
            tiles: vec![vec![Tile::Empty; 1000]; 1000],
            lower_bound: 0
        };
    }

    fn add_path(&mut self, path: &Path) {
        // draw lines between points
        for i in 0..path.points.len() - 1 {
            let p1 = &path.points[i];
            let p2 = &path.points[i + 1];
            if p1.x == p2.x {
                // vertical line
                let (y1, y2) = if p1.y < p2.y { (p1.y, p2.y) } else { (p2.y, p1.y) };
                for y in y1..=y2 {
                    self.tiles[y][p1.x] = Tile::Rock;
                    self.lower_bound = std::cmp::max(self.lower_bound, y);
                }
            } else {
                // horizontal line
                let (x1, x2) = if p1.x < p2.x { (p1.x, p2.x) } else { (p2.x, p1.x) };
                for x in x1..=x2 {
                    self.tiles[p1.y][x] = Tile::Rock;
                    self.lower_bound = std::cmp::max(self.lower_bound, p1.y);
                }
            }
        }
    }

    fn print(&self) {
        for y in 0..10 {
            for x in 490..510 {
                match self.tiles[y][x] {
                    Tile::Empty => print!("."),
                    Tile::Rock => print!("#"),
                    Tile::Sand => print!("o"),
                }
            }
            println!("");
        }
    }

    fn produce_sand_until_flowing(&mut self) -> usize {
        let mut total_units = 0;
        loop {
            if !self.produce_sand_once() {
                break;
            }
            total_units += 1;
        }
        total_units
    }

    fn produce_sand_once(&mut self) -> bool {
        let mut x = 500;
        let mut y = 0;

        loop {
            if y > self.lower_bound {
                return false;
            }

            let tile = &self.tiles[y][x];

            match tile {
                Tile::Rock | Tile::Sand => {
                    let left = &self.tiles[y][x - 1];

                    if left == &Tile::Empty {
                        x -= 1;
                    } else {
                        let right = &self.tiles[y][x + 1];

                        if right == &Tile::Empty {
                            x += 1;
                        } else {
                            self.tiles[y - 1][x] = Tile::Sand;
                            break;
                        }
                    }
                }
                Tile::Empty => y += 1,
            }
        }

        return true;
    }
}

fn main() {
    let input = fs::read_to_string("src/input.txt").unwrap();

    let paths = input
        .trim_end()
        .lines()
        .map(|s| s.parse().unwrap())
        .collect::<Vec<Path>>();

    let mut cave = Cave::new();
    for path in paths.iter() {
        cave.add_path(&path);
    }

    let count = cave.produce_sand_until_flowing();

    println!("count: {}", count);
}
