use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::str::FromStr;
use std::error;
use std::fmt;

#[derive(Debug, PartialEq)]
struct Edge {
  direction: String,
  distance: u32
}

#[derive(Debug, Clone)]
struct EdgeParseError;

impl fmt::Display for EdgeParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Failed to parse edge")
    }
}

// This is important for other errors to wrap this one.
impl error::Error for EdgeParseError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}

impl FromStr for Edge {
  type Err = EdgeParseError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let direction = String::from(s.get(..1).ok_or(EdgeParseError)?);
    let distance : u32 = s[1..].parse::<u32>().or(Err(EdgeParseError))?;

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