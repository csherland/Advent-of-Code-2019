use std::fs;

#[derive(Debug)]
struct Instruction {
    op_code: i32,
    step: usize,
    modes: Vec<i32>
}

fn parse_instruction(mut instruction: i32) -> Instruction {
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

    let mut step: usize = 0;
    match op_code {
        1 | 2 => step = 4,
        3 | 4 => step = 2,
        99 => (), // Program ends, no step
        _ => panic!("Unknown command")
    }

    let mut modes: Vec<i32> = vec![0; step - 1];
    if parameters.len() > 2 {
        for (i, mode) in parameters[2..].iter().enumerate() {
            modes[i] = *mode;
        }
    } 

    Instruction { op_code, step, modes }
}

fn run_intcode_program(mut program: Vec<i32>, input: i32) {
    let mut i = 0;
    while i < program.len() {
        // Identify parameter modes and op code
        let instruction = parse_instruction(program[i]);

        let mut parameters = vec![];
        for (j, mode) in instruction.modes.iter().enumerate() {
            if *mode == 0 {
                parameters.push(program[program[i + j + 1 as usize] as usize]);
            } else {
                parameters.push(program[i + j + 1 as usize]);
            }
        }
        
        let pos = program[i + instruction.step - 1];
        match instruction.op_code {
            1 => program[pos as usize] = parameters[0] + parameters[1],
            2 => program[pos as usize] = parameters[0] * parameters[1],
            3 => program[pos as usize] = input,
            4 => println!("{}", program[parameters[0] as usize]), // Output command
            99 => break,
            _ => panic!("Unknown command")
        }

        i += instruction.step;
    }
}

fn main() {
    let program:Vec<i32> = fs::read_to_string("input.txt")
                                .unwrap()
                                .split(",")
                                .map(|l| l.parse().unwrap())
                                .collect();

    run_intcode_program(program, 1);
}