use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("./src/input.txt").expect("File not found");

    let mut elve_choices: HashMap<String, u32> = HashMap::new();
    let mut round_ends: HashMap<String, u32> = HashMap::new();

    elve_choices.insert("A".to_string(), 0); // rock
    elve_choices.insert("B".to_string(), 1); // paper
    elve_choices.insert("C".to_string(), 2); // scissors
    
    round_ends.insert("X".to_string(), 0); // lose
    round_ends.insert("Y".to_string(), 1); // draw
    round_ends.insert("Z".to_string(), 2); // win

    // my choise based on the matrix [elve_choice][expected_end] matrix
    let my_choices_for_rock = vec![2, 0, 1];
    let my_choices_for_paper = vec![0, 1, 2];
    let my_choices_for_scissors = vec![1, 2, 0];
    let my_choices: Vec<Vec<u32>> = vec![my_choices_for_rock, my_choices_for_paper, my_choices_for_scissors];

    // round points based on [my_choice][elve_choce] matrix
    let rock_results: Vec<u32> = vec![3, 0, 6];
    let paper_results: Vec<u32> = vec![6, 3, 0];
    let scissors_results: Vec<u32> = vec![0, 6, 3];
    let results: Vec<Vec<u32>> = vec![rock_results, paper_results, scissors_results];

    let choice_values: Vec<u32> = vec![1, 2, 3];       

    let mut totals = vec![];

    for s in input.trim_end().split("\n") {
        let encrypted_choices = s.split(" ").collect::<Vec<&str>>();
        let elve_encrypted_choice = encrypted_choices[0];
        let encrypted_expected_end = encrypted_choices[1];
        let elve_choice = elve_choices.get(elve_encrypted_choice).expect("Elve choice not found").clone();
        let expected_end = round_ends.get(encrypted_expected_end).expect("encrypted end not found").clone();
        let my_choice = my_choices[elve_choice as usize][expected_end as usize];

        let result = &results[my_choice as usize][elve_choice as usize];
        let match_total = result + choice_values[my_choice as usize];
        totals.push(match_total);
    }

    
    println!("{}", totals.iter().sum::<u32>())
}
