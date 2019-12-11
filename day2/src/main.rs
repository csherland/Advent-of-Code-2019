use std::fs;

fn main() {
    let mut commands:Vec<i32> = fs::read_to_string("input.txt")
                                .unwrap()
                                .split(",")
                                .map(|l| l.parse().unwrap())
                                .collect();

    for x in 0..100 {
        for y in 0..100 {
            let mut commands_copy = commands.clone();
            commands_copy[1] = x;
            commands_copy[2] = y;

            for command in commands_copy.clone().chunks(4) {
                if x as usize > commands.len() || y as usize > commands.len() || command[3] as usize > commands.len() {
                    break;
                }
                match command {
                    [1, x, y, pos] => commands[*pos as usize] = commands[*x as usize] + commands[*y as usize],
                    [2, x, y, pos] => commands[*pos as usize] = commands[*x as usize] * commands[*y as usize],
                    [99, _, _, _] => break,
                    _ => ()
                }
            }

            if commands_copy[0] == 19_690_720 {
                println!("Soulution found, x: {}, y: {}", x, y);
                break;
            }
        }
    }
}