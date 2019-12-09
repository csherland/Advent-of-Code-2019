use std::fs;

fn main() {
    let mut commands:Vec<u32> = fs::read_to_string("input.txt")
                                .unwrap()
                                .split(",")
                                .map(|l| l.parse().unwrap())
                                .collect();

    for i in 0..commands.len() {
        
        // Identify command modes and op code
        let command = parseCommand(commands[i]);

        match operation {
            1 => commands[*pos as usize] = commands[*x as usize] + commands[*y as usize],
            2 => commands[*pos as usize] = commands[*x as usize] * commands[*y as usize],
            3 => commands[*pos as usize] = input,
            4 => input = commands[*pos as usize],
            99 => break,
            _ => panic!("Uknown command")
        }

        // Figure out how to do this properly
        for j in 0..command.len() {
            i.next();
        }
    }

    println!("{:?}", commands)
}

fn parseCommand(command: u32) -> Vec<u32> {
    let mut res = vec![];

    let mut i = command;
    while i > 0 {
        res.push(i % 10);
        i /= 10;
    }
    res.reverse();

    res
}