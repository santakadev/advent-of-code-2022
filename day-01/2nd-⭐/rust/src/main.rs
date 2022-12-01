use std::collections::BinaryHeap;
use std::fs::File;
use std::io::{BufReader, BufRead, self};

fn main() -> io::Result<()> {
    // read data [[1,2,3], [134,2352,235], [...]] format
    let file = File::open("./src/input.txt")?;
    let reader = BufReader::new(file);

    let mut elves = Vec::new();
    let mut elve = Vec::new();

    for line in reader.lines() {
        let calories: String = line?;
        if calories == "" {
            elves.push(elve);
            elve = Vec::new();
        } else {
            elve.push(calories.parse::<u32>().unwrap())
        }
    }

    // sum top3 calories
    let mut heap = BinaryHeap::new();
    for elve in elves {

        let total_calories: u32 = elve.iter().sum();
        heap.push(total_calories);
    }

    let top1 = heap.pop().unwrap();
    let top2 = heap.pop().unwrap();
    let top3 = heap.pop().unwrap();

    println!("{}", top1 + top2 + top3);

    Ok(())
}
