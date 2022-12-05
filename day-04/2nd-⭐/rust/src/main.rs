use std::str::FromStr;

#[derive(Debug)]
struct ElvesPair {
    first: Elve,
    second: Elve,
}

#[derive(Debug)]
struct Elve {
    min: u32,
    max: u32,
}

impl FromStr for ElvesPair {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split(",");
        let first = Elve::from_str(iter.next().unwrap()).unwrap();
        let second = Elve::from_str(iter.next().unwrap()).unwrap();
        Ok(ElvesPair { first, second })
    }
}

impl Elve {
    fn overlaps(&self, other: &Elve) -> bool {
        self.min <= other.max && other.min <= self.max
    }
}

impl ElvesPair {
    fn overlaps(&self) -> bool {
        self.first.overlaps(&self.second)
    }
}

impl FromStr for Elve {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split("-");
        let min = u32::from_str(iter.next().unwrap()).unwrap();
        let max = u32::from_str(iter.next().unwrap()).unwrap();
        Ok(Elve { min, max })
    }
}

fn main() {
    let input = std::fs::read_to_string("src/input.txt").unwrap();

    let solution = input
        .trim_end()
        .split("\n")
        .map(|line| line .parse::<ElvesPair>() .unwrap())
        .map(|pair| pair.overlaps())
        .filter(|overlaps| *overlaps)
        .count();

    println!("{:?}", solution);
}

