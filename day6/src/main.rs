use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::collections::HashMap;

fn main() {
    let fp = File::open("./input.txt")
                    .expect("Could not find file");

    let file = BufReader::new(&fp);

    let map : Vec<String> = file.lines()
                .map(|l| l.expect("Could not read line").parse().unwrap())
                .collect();

    // Create the orbit list
    let mut orbits : HashMap<String, Vec<String>> = HashMap::new();
    for orbit in map.clone() {
        let o: Vec<&str> = orbit.split(")").collect();
        let planet = String::from(o[0]);
        let moon = String::from(o[1]);

        match orbits.get(&planet) {
            Some(o) => {
                let mut new_moon = o.clone();
                new_moon.push(moon);
                orbits.insert(planet, new_moon);
            },
            None => {
                orbits.insert(planet, vec![String::from(moon.clone())]);
            }
        }

        match orbits.get(&moon) {
            Some(_) => (),
            None => {
                orbits.insert(String::from(moon.clone()), vec![]);
            }
        }
    }

    let (direct, indirect) = count_orbits(orbits, "COM", 0);

    println!("Direct Orbits: {:?}", direct);
    println!("Indirect Orbits: {:?}", indirect);
    println!("Total: {:?}", direct + indirect);
}

fn count_orbits(orbits: HashMap<String, Vec<String>>, name: &str, level: u32) -> (u32, u32) {
    let mut direct = 0;
    let mut indirect = 0;

    let orbitals;
    match orbits.get(name) {
        Some(o) => orbitals = o,
        None => return (direct, indirect)
    }

    for orbital in orbitals {
        direct += 1;
        indirect += level;
        let (d, i) = count_orbits(orbits.clone(), orbital, level + 1);
        direct += d;
        indirect += i;
    }

    (direct, indirect)
}