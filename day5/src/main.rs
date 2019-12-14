use std::fs;

#[derive(Debug)]
struct Instruction {
    op_code: i64,
    step: usize,
    modes: Vec<i64>
}

fn parse_instruction(mut instruction: i64) -> Instruction {
    let mut parameters = vec![];
    while instruction > 0 {
        parameters.push(instruction % 10);
        instruction /= 10;
    }

    let op_code;
    if parameters.len() > 1 {
        op_code = parameters[0] + 10 * parameters[1];
    } else {
        op_code = parameters[0];
    }

    let step: usize;
    match op_code {
        1 | 2 | 7 | 8  => step = 4,
        3 | 4 => step = 2,
        5 | 6 => step = 3,
        99 => step = 2, // Program ends, no step
        _ => panic!("Unknown command")
    }

    let mut modes: Vec<i64> = vec![0; step - 1];
    if parameters.len() > 2 {
        for (i, mode) in parameters[2..].iter().enumerate() {
            modes[i] = *mode;
        }
    } 

    Instruction { op_code, step, modes }
}

fn run_intcode_program(mut program: Vec<i64>, input: i64) -> i64{
    let mut output: i64 = 0;
    let mut i = 0;
    while i < program.len() {
        // Identify parameter modes and op code
        let instruction = parse_instruction(program[i]);

        if instruction.op_code == 99 {
            break;
        }

        let mut parameters = vec![];

        for (j, mode) in instruction.modes.iter().enumerate() {
            if *mode == 0 {
                parameters.push(program[program[i + j + 1 as usize] as usize]);
            } else {
                parameters.push(program[i + j + 1 as usize]);
            }
        }
        
        let pos = program[i + instruction.step - 1] as usize;
        match instruction.op_code {
            1 => program[pos] = parameters[0] + parameters[1],
            2 => program[pos] = parameters[0] * parameters[1],
            3 => program[pos] = input,
            4 => output = parameters[0],
            5 => if parameters[0] != 0 { i = parameters[1] as usize; continue; },
            6 => if parameters[0] == 0 { i = parameters[1] as usize; continue; },
            7 => if parameters[0] < parameters[1] { program[pos] = 1; } else { program[pos] = 0; },
            8 => if parameters[0] == parameters[1] { program[pos] = 1; } else { program[pos] = 0; },
            _ => panic!("Unknown command")
        }

        i += instruction.step;
    }
    
    output
}

fn main() {
    let program1:Vec<i64> = fs::read_to_string("input_1.txt")
                                .unwrap()
                                .split(",")
                                .map(|l| l.parse().unwrap())
                                .collect();

    println!("Result 1: {}", run_intcode_program(program1, 1));

    let program2:Vec<i64> = fs::read_to_string("input_2.txt")
                                .unwrap()
                                .split(",")
                                .map(|l| l.parse().unwrap())
                                .collect();

    println!("Result 2: {}", run_intcode_program(program2, 5));
}