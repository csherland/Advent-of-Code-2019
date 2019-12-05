use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::str::FromStr;
use std::error;
use std::fmt;

#[derive(Debug, PartialEq)]
struct Edge {
  direction: String,
  length: i32,
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
    let length : i32 = s[1..].parse::<i32>().or(Err(EdgeParseError))?;
    let start = Point { x: 0, y: 0 }; // populated on second run through array

    Ok(Edge { direction, length, start })
  }
}

impl Edge {
  fn get_points(&self) -> Vec<Point> {

    let mut points: Vec<Point> = vec![];

    // todo: messy
    match self.direction.as_ref() {
      "U" => for y in self.start.y+1..self.start.y+self.length+1 { points.push(Point { x: self.start.x, y })},
      "D" => for y in self.start.y-self.length..self.start.y { points.push(Point { x: self.start.x, y })},
      "L" => for x in self.start.x-self.length..self.start.x { points.push(Point { x, y: self.start.y })},
      "R" => for x in self.start.x+1..self.start.x+self.length+1 { points.push(Point { x, y: self.start.y })},
      _ => panic!("Direction invalid!")
    }

    match self.direction.as_ref() {
      "D" | "L" => points.reverse(),
      _ => ()
    }

    points.clone()
  }

  fn get_end(&self) -> Option<Point> {
    self.get_points().last().cloned()
  }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Clone, Copy)]
struct Point {
  x: i32,
  y: i32
}

fn main() {
  let fp = File::open("./input.txt")
                .expect("Could not find input file");

  let file = BufReader::new(&fp);

  let mut points = file.lines()
                  .map(|l| l.expect("Wire invalid").split(",").map(|s| s.parse::<Edge>().unwrap()).collect())
                  .fold(vec![], | mut acc, mut wire: Vec<Edge> | { // Set starts for vectors
                    let mut start = Point { x: 0, y: 0 };
                    for edge in &mut wire {
                      (*edge).start = start;
                      start = edge.get_end().unwrap();
                    } 
                    acc.push(wire);
                    acc
                  })
                  .iter()
                  .fold(vec![], | mut acc: Vec<Point>, wire: &Vec<Edge> | {
                    for edge in wire {
                      acc.append(&mut edge.get_points())
                    }
                    acc
                  });


  points.sort_by(|a, b| (i32::abs(a.x) + i32::abs(a.y)).cmp(&(i32::abs(b.x) + i32::abs(b.y))));

  let mut intersections = vec![];
  for (i, point_a) in points.iter().enumerate() {
    for j in i+1..points.len() {
      let point_b = points[j];

      if (*point_a).x == point_b.x && (*point_a).y == point_b.y {
        intersections.push(*point_a);
        break;
      }
    }
  }

  intersections.sort_by(|a, b| (i32::abs(a.x) + i32::abs(a.y)).cmp(&(i32::abs(b.x) + i32::abs(b.y))));

  println!("{:?}", intersections);
}