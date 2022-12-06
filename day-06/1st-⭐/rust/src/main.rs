use std::{fs, collections::HashSet};

fn main() {
    let input = fs::read_to_string("src/input.txt").unwrap();

    for i in 0..input.len() {
        // let mut marker: Vec<char> = input[i..i+4].chars().collect();
        // marker.sort();
        // marker.dedup();
        let marker: HashSet<char> = input[i..i+4].chars().collect();

        if marker.len() == 4 {
            println!("{}", i + 4);
            break;
        }
    }
}
