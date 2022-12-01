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

    // compute and save max
    let mut max_calories = 0;
    for elve in elves {
        let total_calories: u32 = elve.iter().sum();
        max_calories = std::cmp::max(total_calories, max_calories)
    }

    println!("{}", max_calories);

    Ok(())
}
