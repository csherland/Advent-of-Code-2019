use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
  let fp = File::open("./inputs.txt")
                .expect("Could not find file");

  let file = BufReader::new(&fp);

  let inputs : Vec<i32> = file.lines()
                  .map(|l| l.expect("Could not read line").parse().unwrap())
                  .collect();

  let mut required_fuel = 0; 
  for mass in inputs.clone() {
    let mut fuel = ( mass / 3 ) - 2;
    required_fuel += fuel;

    // Mass of fuel we just added also requires fuel!
    while fuel > 0 {
      let extra_fuel = (fuel / 3) - 2;
      if extra_fuel > 0 {
        required_fuel += extra_fuel;
      }
      fuel = extra_fuel;
    }

  }

  println!("Required fuel: {}", required_fuel);
}