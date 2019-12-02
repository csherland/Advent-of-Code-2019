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

  for mass in inputs {
    required_fuel += ( mass / 3 ) - 2;
  }

  println!("Required fule: {}", required_fuel);
}