use std::collections::HashSet;
use std::char;
use std::str::FromStr;

struct Rucksack {
   compartment_a: Compartment,
   compartment_b: Compartment,
}

struct Compartment {
    items: HashSet<char>
}

impl FromStr for Compartment {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let items = s.chars().collect();
        Ok(Compartment { items })
    }
}

impl FromStr for Rucksack {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (c1, c2) = s.split_at(s.len() / 2);

        
        let items_c1: HashSet<char> = c1.chars().collect();
        let items_c2: HashSet<char> = c2.chars().collect();

        Ok(Rucksack {
            compartment_a: Compartment { items: items_c1 },
            compartment_b: Compartment { items: items_c2 }
        })
    }
}

impl Rucksack {
    fn repeated_item_priority(&self) -> i32 {
        let repeated_item_priority: i32 = self.compartment_a.first_repeated_item(&self.compartment_b) as i32;
        if repeated_item_priority > 90 { repeated_item_priority - 96} else { repeated_item_priority - 64 + 26 }
    }
}

impl Compartment {
    fn first_repeated_item(&self, other: &Compartment) -> char {
        let intersection = self.items.intersection(&other.items);
        *intersection.into_iter().next().unwrap()
    }
}

fn main() {
    let input = std::fs::read_to_string("./src/input.txt")
        .expect("Error while reading the file");

    let repeated_prios: i32 = input
        .trim_end()
        .split("\n")
        .map(|s| Rucksack::from_str(s).expect("Can't parse from string"))
        .map(|rucksack| rucksack.repeated_item_priority())
        .sum();

    println!("{:?}", repeated_prios);
}

