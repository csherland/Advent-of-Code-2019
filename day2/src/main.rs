use std::fs;

fn main() {
    let mut commands:Vec<i32> = fs::read_to_string("input.txt")
                                .unwrap()
                                .split(",")
                                .map(|l| l.parse().unwrap())
                                .collect();

    for command in commands.chunks(4) {
        match command {
            [1, x, y, pos] => commands[*pos as usize] = *x + *y,
            [2, x, y, pos] => commands[*pos as usize] = *x * *y,
            [99, _, _, _] => break,
            _ => panic!("Unrecognized command")
        }
    }
}