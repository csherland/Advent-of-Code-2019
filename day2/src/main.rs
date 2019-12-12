use std::fs;

fn main() {
    let commands:Vec<i32> = fs::read_to_string("input.txt")
                                .unwrap()
                                .split(",")
                                .map(|l| l.parse().unwrap())
                                .collect();

    for x in 0..100 {
        for y in 0..100 {
            let mut commands_copy = commands.clone();
            commands_copy[1] = x;
            commands_copy[2] = y;

            let result = run_int_code_program(commands_copy);

            if result[0] == 19_690_720 {
                panic!("Soulution found, x: {}, y: {}", x, y);
            }
        }
    }
}

fn run_int_code_program(mut program: Vec<i32>) -> Vec<i32> {
    for command in program.clone().chunks(4) {
        if command[1] as usize > program.len() || command[2] as usize > program.len() || command[3] as usize > program.len() {
            break;
        }
        match command {
            [1, x, y, pos] => program[*pos as usize] = program[*x as usize] + program[*y as usize],
            [2, x, y, pos] => program[*pos as usize] = program[*x as usize] * program[*y as usize],
            [99, _, _, _] => break,
            _ => ()
        }
    }

    program
}