use std::fs;

struct Instruction {
    op_code: i32,
    step: i32,
    modes: Vec<i32>
}

fn parseInstruction(instruction: i32) -> Instruction {
    let mut res = vec![];

    let mut i = instruction;
    while i > 0 {
        res.push(i % 10);
        i /= 10;
    }

    let op_code = 0;
    let modes = vec![];

    let mut step = 0;
    match op_code {
        1 | 2 => step = 4,
        3 | 4 => step = 2,
        99 => (),
        _ => panic!("Unrecognized operation code")
    }

    Instruction { op_code, step, modes }
}

fn main() {
    let mut commands:Vec<i32> = fs::read_to_string("input.txt")
                                .unwrap()
                                .split(",")
                                .map(|l| l.parse().unwrap())
                                .collect();
    let mut input = 1;

    let mut i = 0;
    while i < commands.len() {
        // Identify parameter modes and op code
        let instruction = parseInstruction(commands[x]);

        let mut x:i32 = 0;
        let mut y:i32 = 0;
        let mut pos:usize = 0;

        match instruction.op_code {
            1 => commands[pos as usize] = commands[x as usize] + commands[y as usize],
            2 => commands[pos as usize] = commands[x as usize] * commands[y as usize],
            3 => commands[pos as usize] = input,
            4 => println!("{}", commands[pos as usize]), // Output command
            99 => break,
            _ => panic!("Uknown command")
        }

        i += instruction.step as usize;
    }

}