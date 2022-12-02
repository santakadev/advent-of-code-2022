use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("./src/input.txt").expect("File not found");

    let mut elve_choices: HashMap<String, u32> = HashMap::new();
    let mut my_choices: HashMap<String, u32> = HashMap::new();

    elve_choices.insert("A".to_string(), 0);
    elve_choices.insert("B".to_string(), 1);
    elve_choices.insert("C".to_string(), 2);
    
    my_choices.insert("X".to_string(), 0);
    my_choices.insert("Y".to_string(), 1);
    my_choices.insert("Z".to_string(), 2);

    let rock_results: Vec<u32> = vec![3, 0, 6];
    let paper_results: Vec<u32> = vec![6, 3, 0];
    let scissors_results: Vec<u32> = vec![0, 6, 3];
    let results: Vec<Vec<u32>> = vec![rock_results, paper_results, scissors_results];

    let choice_values: Vec<u32> = vec![1, 2, 3];       

    let mut totals = vec![];

    for s in input.trim_end().split("\n") {
        let encrypted_choices = s.split(" ").collect::<Vec<&str>>();
        let elve_encrypted_choice = encrypted_choices[0];
        let my_encrypted_choice = encrypted_choices[1];
        let elve_choice = elve_choices.get(elve_encrypted_choice).expect("Elve choice not found").clone();
        let my_choice = my_choices.get(my_encrypted_choice).expect("My choice not found").clone();

        let result = &results[my_choice as usize][elve_choice as usize];
        let match_total = result + choice_values[my_choice as usize];
        totals.push(match_total);
    }

    
    println!("{}", totals.iter().sum::<u32>())
}
