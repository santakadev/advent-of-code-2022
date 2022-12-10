use std::{fs, str::FromStr};
use std::str;

#[derive(Debug)]
enum Instruction {
    Noop(),
    Addx(i32)
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (instruction, value) = s.split_at(4);
        match instruction.trim() {
            "noop" => Ok(Instruction::Noop()),
            "addx" => Ok(Instruction::Addx(value.trim().parse::<i32>().unwrap())),
            _ => panic!("Unknown instruction")
        }
    }
}

fn main() {
    let input = fs::read_to_string("src/input.txt").unwrap();

    input.trim_end().lines().map(|line| Instruction::from_str(line).unwrap()).flat_map(|op| {
        match op {
            Instruction::Noop() => [0].to_vec(),
            Instruction::Addx(x) => [0, x].to_vec()
        }
    }).enumerate().fold(1, |sprite_pos, (cycle, value)|  {
        if ((cycle % 40) as i32 - sprite_pos).abs() < 2 {
            print!("#");
        } else {
            print!(".");
        }
        if (cycle + 1) % 40 == 0 {
            println!();
        }
        return sprite_pos + value;
    });
}
