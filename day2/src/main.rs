use std::fs;

fn main() {
    let mut commands:Vec<i32> = fs::read_to_string("input.txt")
                                .unwrap()
                                .split(",")
                                .map(|l| l.parse().unwrap())
                                .collect();

    for command in commands.clone().chunks(4) {
        match command {
            [1, x, y, pos] => commands[*pos as usize] = commands[*x as usize] + commands[*y as usize],
            [2, x, y, pos] => commands[*pos as usize] = commands[*x as usize] * commands[*y as usize],
            [99, _, _, _] => break,
            _ => panic!("Unrecognized command")
        }
    }

    println!("{:?}", commands)
}