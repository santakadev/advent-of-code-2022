use std::{fs, str::FromStr};

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

    let ops = input.trim_end().lines().map(|line| Instruction::from_str(line).unwrap()).map(|op| {
        match op {
            Instruction::Noop() => (1, 0),
            Instruction::Addx(x) => (2, x)
        }
    }).collect::<Vec<(i32, i32)>>();
    
    let mut register = 1;
    let mut target_cycle = 20;
    let mut current_cycle = 0;
    let mut result = 0;

    for (op_count, op_value) in ops {
        if (current_cycle + op_count) >= target_cycle {
            result += target_cycle * register;
            target_cycle += 40;
        }

        register += op_value;
        current_cycle += op_count;
    }

    println!("Result: {}", result);
}
