use std::collections::HashSet;
use std::char;
use std::str::FromStr;

#[derive(Debug)]
struct Rucksack {
    items: HashSet<char>
}

#[derive(Debug)]
struct Item {
    item: char
}

impl FromStr for Rucksack {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let items = s.chars().collect();
        Ok(Rucksack { items })
    }
}

impl Item {
    fn new(c: char) -> Self {
        Self { item: c }
    }

    fn priority(&self) -> i32 {
        let priority: i32 = self.item as i32;
        if priority > 90 { priority - 96} else { priority - 64 + 26 }
    }
}

impl Rucksack {
    fn intersect(&self, other: Rucksack) -> Rucksack {
        let intersection: HashSet<char> = self.items.intersection(&other.items).map(|c| *c).collect::<HashSet<char>>();
        Rucksack { items: intersection } 
    }

    fn fisrt_item(&self) -> Item {
        Item { item: *self.items.iter().next().unwrap() }

    }
}

fn main() {
    let input = std::fs::read_to_string("./src/input.txt")
        .expect("Error while reading the file");

    let grouped: i32 = input
        .trim_end()
        .split("\n")
        .map(|s| Rucksack::from_str(s).unwrap())
        .fold(Vec::new() as Vec<Vec<Rucksack>>, |mut acc, l| {
            if acc.last().is_some() && acc.last().unwrap().len() < 3 {
                acc.last_mut().unwrap().push(l);
            } else {
                acc.push(vec![l]);
            }
            acc
        })
        .into_iter()
        .flat_map(|group| group.into_iter().reduce(|acc, r| r.intersect(acc)))
        .map(|b| b.fisrt_item().priority()).sum();


    println!("{:?}", grouped);
}

