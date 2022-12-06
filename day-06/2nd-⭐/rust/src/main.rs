use std::{fs, collections::HashSet};

fn main() {
    let input = fs::read_to_string("src/input.txt").unwrap();

    let marker_size = 14;

    for i in 0..input.len() {
        let marker: HashSet<char> = input[i..i+marker_size].chars().collect();

        if marker.len() == marker_size {
            println!("{}", i + marker_size);
            break;
        }
    }
}
