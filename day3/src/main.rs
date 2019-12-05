use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::str::FromStr;
use std::num::ParseIntError;

#[derive(Debug, PartialEq)]
struct Edge {
  direction: String,
  distance: u32
}

impl FromStr for Edge {
  type Err = ParseIntError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let direction = String::from(s);
    let distance : u32 = 5;

    Ok(Edge { direction, distance })
  }
}

fn main() {
  let fp = File::open("./input.txt")
                .expect("Could not find file");

  let file = BufReader::new(&fp);

  let wires : Vec<Vec<Edge>> = file.lines()
                  .map(|l| l.expect("Could not read line").split(",").map(|s| s.parse::<Edge>().unwrap()).collect())
                  .collect();

  println!("{:?}", wires);
}