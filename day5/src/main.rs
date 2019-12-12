use std::fs;

struct Instruction {
    op_code: i32,
    step: i32,
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

    let modes = vec![];

    let mut step = 0;
    match op_code {
        1 | 2 => step = 4,
        3 | 4 => step = 2,
        99 => (), // Program ends, no step
        _ => panic!("Unknown command")
    }

    Instruction { op_code, step, modes }
}

fn run_intcode_program(mut program: Vec<i32>, input: i32) -> Vec<i32> {
    let mut i = 0;
    while i < program.len() {
        // Identify parameter modes and op code
        let instruction = parse_instruction(program[i]);

        // Need to set these based on instruction
        let mut x:i32 = 0;
        let mut y:i32 = 0;
        let mut pos:usize = 0;


        match instruction.op_code {
            1 => program[pos] = x + y,
            2 => program[pos] = x * y,
            3 => program[pos] = input,
            4 => println!("{}", program[pos]), // Output command
            99 => break,
            _ => panic!("Unknown command")
        }

        i += instruction.step as usize;
    }

    program
}

fn main() {
    let program:Vec<i32> = fs::read_to_string("input.txt")
                                .unwrap()
                                .split(",")
                                .map(|l| l.parse().unwrap())
                                .collect();

    let result = run_intcode_program(program, 1);

    println!("Program result: {}", result[0]);
}