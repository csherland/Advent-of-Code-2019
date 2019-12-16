use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

#[derive(Debug)]
struct Orbit {
    name: String,
    orbitals: Vec<Orbit>
} 

fn main() {
    let fp = File::open("./input.txt")
                    .expect("Could not find file");

    let file = BufReader::new(&fp);

    let map : Vec<String> = file.lines()
                .map(|l| l.expect("Could not read line").parse().unwrap())
                .collect();

    let mut orbits : Vec<Orbit> = vec![];
    for orbit in map {
        let (name, moon) = orbit.split_at(orbit.find(")").unwrap());

        let child;
        match orbits.iter().find(|&x| x.name == name) {
            Some(o) => child = *o,
            None => { 
                child = Orbit { name: String::from(moon), orbitals: vec![] };
                orbits.push(child);
            }
        }

        println!("{:?}", child);

        // parent.orbitals.push(child);
    }

    // let center_of_mass = get_orbit(orbits, String::from("COM"));

    // println!("{:?}", center_of_mass);
}