use std::str::{self, FromStr};

#[derive(Debug)]
struct Crane {
    stacks: Vec<CrateStack>,
}

impl Crane {
    fn move_crane(&mut self, from: usize, to: usize) {
        let crane = self.stacks[from - 1].pop().expect("No crates to move");
        self.stacks[to - 1].push(crane);
    }

    fn move_crane_multiple(&mut self, from: usize, to: usize, count: usize) {
        for _ in 0..count {
            self.move_crane(from, to);
        }
    }
}


impl FromStr for Crane {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut crates: Vec<&str> = s.split("\n").collect();
        crates.pop();
        crates.reverse();

        let stack_count = (crates[0].len() + 1) / 4;
        let mut stacks = (0..stack_count).map(|_| CrateStack::new()).collect::<Vec<CrateStack>>();

        let parts2 = crates.iter().map(|line| line.as_bytes().chunks(4).map(|c| str::from_utf8(c).unwrap().trim()).collect::<Vec<&str>>()).collect::<Vec<Vec<&str>>>();
        
        for line in &parts2 {
            for (i, part) in line.iter().enumerate() {
                if part.len() > 0 {
                    let c = Crate { id: part.chars().nth(1).unwrap() };
                    stacks[i].push(c);
                }
            }
        }

        Ok(Self { stacks })
    }
}

#[derive(Debug)]
struct CrateStack {
    items: Vec<Crate>
}

impl CrateStack {
    fn new() -> Self {
        Self { items: Vec::new() }
    }

    fn push(&mut self, c: Crate) {
        self.items.push(c);
    }

    fn pop(&mut self) -> Option<Crate> {
        self.items.pop()
    }
}

#[derive(Debug)]
struct Crate {
    id: char
}

#[derive(Debug)]
struct Operation {
    crate_from: u32,
    crate_to: u32,
    amount: u32
}

fn main() {
    let input = std::fs::read_to_string("src/input.txt").unwrap();

    let parts = input.split("\n\n").collect::<Vec<&str>>();

    // Parse stacks
    let mut crane: Crane = parts[0].parse().unwrap();

    // Parse moves
    let mut ops: Vec<Operation> = Vec::new();
    let ops_str = parts[1].trim_end().split("\n").collect::<Vec<&str>>();
    let re = regex::Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    for line in ops_str {
        let caps = re.captures(line).unwrap();
        let op = Operation {
            amount: u32::from_str(caps.get(1).unwrap().as_str()).unwrap(),
            crate_from: u32::from_str(caps.get(2).unwrap().as_str()).unwrap(),
            crate_to: u32::from_str(caps.get(3).unwrap().as_str()).unwrap()
        };
        ops.push(op);
    }

    // Execute moves
    for op in ops {
        crane.move_crane_multiple(op.crate_from as usize, op.crate_to as usize, op.amount as usize);
    }
    
    // Print last char of each stack
    for stack in &crane.stacks {
        let c = stack.items.last().unwrap();
        print!("{}", c.id);
    }
}

