use std::{fs, str::FromStr, collections::VecDeque};

#[derive(Debug, Clone)]
struct Item {
    worry_level: u32
}

impl FromStr for Item {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let worry_level = s.parse().unwrap();
        Ok(Item { worry_level })
    }
}

impl Item {
    fn new(worry_level: u32) -> Self {
        Item { worry_level }
    }
}

struct Operation {
    left_token: String,
    operation_token: char,
    right_token: String
}

impl Operation {
    fn execute(&self, item: &Item) -> u32 {
        let left = if self.left_token == "old" { item.worry_level } else { self.left_token.parse().unwrap() };
        let right = if self.right_token == "old" { item.worry_level } else { self.right_token.parse().unwrap() };
        match self.operation_token {
            '*' => left * right,
            '+' => left + right,
            _ => panic!("Unknown operation: {}", self.operation_token)
        }
    }
}

struct Monkey {
    items: VecDeque<Item>,
    operation: Operation,
    divisible_by: u32,
    true_monkey: u32,
    false_monkey: u32,
    inspections: u32
}

impl FromStr for Monkey {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = regex::Regex::new(r"Starting items: (.*)\n").unwrap();
        let caps = re.captures(s).unwrap();
        let items = caps.get(1).unwrap().as_str().split(",").map(|x| x.trim().parse().unwrap()).collect::<VecDeque<Item>>();

        let re = regex::Regex::new(r"Operation: new = (old|\d+) (\*|\+) (old|\d+)\n").unwrap();
        let caps = re.captures(s).unwrap();
        let (lhs, op, rhs) = (caps.get(1).unwrap().as_str(), caps.get(2).unwrap().as_str(), caps.get(3).unwrap().as_str());
        let operation = Operation {
            left_token: lhs.parse().unwrap(),
            operation_token: op.parse().unwrap(),
            right_token: rhs.parse().unwrap()
        };
        
        let re = regex::Regex::new(r"Test: divisible by (\d+)\n").unwrap();
        let caps = re.captures(s).unwrap();
        let divisible_by = caps.get(1).unwrap().as_str().parse().unwrap();
        
        let re = regex::Regex::new(r"If true: throw to monkey (\d+)\n").unwrap();
        let caps = re.captures(s).unwrap();
        let true_monkey = caps.get(1).unwrap().as_str().parse().unwrap();

        let re = regex::Regex::new(r"If false: throw to monkey (\d+)").unwrap();
        let caps = re.captures(s).unwrap();
        let false_monkey = caps.get(1).unwrap().as_str().parse().unwrap();

        Ok(Monkey { 
            items,
            operation,
            divisible_by,
            true_monkey,
            false_monkey,
            inspections: 0
        })
    }
}

impl Monkey {
    fn observe_and_trow(&mut self) -> Vec<ThrowDescriptor> {
        let items: Vec<Item> = self.items.iter()
            .map(|x| Item::new(self.operation.execute(x) / 3))
            .collect();

        self.inspections += items.len() as u32;
        
        self.items.clear();

        return items.iter()
            .map(|item|  {
                if item.worry_level % self.divisible_by == 0 { 
                    ThrowDescriptor::new(item, self.true_monkey)
                } else { 
                    ThrowDescriptor::new(item, self.false_monkey)
                }
            }).collect();
    }
}

struct ThrowDescriptor {
    item: Item,
    to_monkey: u32
}

impl ThrowDescriptor {
    fn new(item: &Item, to_monkey: u32) -> Self { ThrowDescriptor { 
            item: Item::new(item.worry_level),
            to_monkey
        }
    }
}

struct Round {
    monkeys: Vec<Monkey>
}

impl Round {
    fn run_once(&mut self) {
        for i in 0..self.monkeys.len() {
            let throws = self.monkeys[i].observe_and_trow();
            for t in throws {
                self.monkeys[t.to_monkey as usize].items.push_back(t.item);
            }
        }
    }

    fn run(&mut self, times: u32) {
        for _ in 0..times {
            self.run_once();
        }
    }

    fn monkey_business(&mut self) -> u32 {
        self.monkeys.sort_by(|a, b| b.inspections.cmp(&a.inspections));
        return self.monkeys[0..2].iter().map(|x| x.inspections).reduce(|a, b| a * b).unwrap();
    }
}

impl FromStr for Round {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let monkeys = s.trim_end().split("\n\n").map(|line| line.parse::<Monkey>().unwrap()).collect();
        Ok(Round { monkeys })
    }
}
    

fn main() {
    let input = fs::read_to_string("src/input.txt").unwrap();

    let mut round = Round::from_str(&input).unwrap();
    round.run(20);

    let state = round.monkeys.iter()
        .map(|m| m.items.iter().map(|i| i.worry_level).collect::<Vec<u32>>())
        .collect::<Vec<Vec<u32>>>();

    println!("Monkey business {:?}", round.monkey_business());
}
