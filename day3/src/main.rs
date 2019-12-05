use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::str::FromStr;
use std::error;
use std::fmt;
use std::cmp::Ordering;

#[derive(Debug, PartialEq)]
struct Edge {
  direction: String,
  length: u32,
  start: Point
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
    let length : u32 = s[1..].parse::<u32>().or(Err(EdgeParseError))?;
    let start = Point { x: 0, y: 0 }; // TODO

    Ok(Edge { direction, length, start })
  }
}

impl Edge {
  fn getPoints(&self) -> Vec<Point> {

    let mut points: Vec<Point> = vec![];

    // todo: messy
    match self.direction.as_ref() {
      "U" => for y in self.start.y-self.length..self.start.y { points.push(Point { x: self.start.x, y })},
      "D" => for y in self.start.y..self.start.y+self.length { points.push(Point { x: self.start.x, y })},
      "L" => for x in self.start.x-self.length..self.start.x { points.push(Point { x, y: self.start.y })},
      "R" => for x in self.start.x..self.start.x+self.length { points.push(Point { x, y: self.start.y })},
      _ => panic!("Direction invalid!")
    }

    points
  }

  fn getEnd(&self) -> Option<Point> {
    self.getPoints().last().cloned()
  }

  fn intersection(&self, edge: Edge) -> Option<Point>{
    for point_x in self.getPoints() {
      for point_y in edge.getPoints() {
        if point_x == point_y {
          Some(point_x);
        }
      }
    }

    None 
  }
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Point {
  x: u32,
  y: u32
}

impl Ord for Point {
  fn cmp(&self, other: &Self) -> Ordering {
    (self.x + self.y).cmp(&(other.x + other.y))
  }
}

fn main() {
  let fp = File::open("./input.txt")
                .expect("Could not find input file");

  let file = BufReader::new(&fp);

  let wires : Vec<Vec<Edge>> = file.lines()
                  .map(|l| l.expect("Wire invalid").split(",").map(|s| s.parse::<Edge>().unwrap()).collect())
                  .collect(); // Vec<Vec<Edge>>
                  // .fold() // Vec<Point>

  
  println!("{:?}", wires);
}