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
  points: Vec<Point>
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
    Ok(Edge { direction, length, points: vec![] })
  }
}

impl Edge {
  fn get_points(&self, start: Point) -> Vec<Point> {

    let mut points: Vec<Point> = vec![];

    // todo: messy
    match self.direction.as_ref() {
      "U" => for y in start.y+1..start.y+self.length+1 { points.push(Point { x: start.x, y, steps: 0})},
      "D" => for y in start.y-self.length..start.y { points.push(Point { x: start.x, y, steps: 0})},
      "L" => for x in start.x-self.length..start.x { points.push(Point { x, y: start.y, steps: 0})},
      "R" => for x in start.x+1..start.x+self.length+1 { points.push(Point { x, y: start.y, steps: 0 })},
      _ => panic!("Direction invalid!")
    }

    match self.direction.as_ref() {
      "D" | "L" => points.reverse(),
      _ => ()
    }

    let mut steps = start.steps;
    let mut points_with_steps = vec![];
    for mut point in points {
      steps += 1;
      point.steps = steps;
      points_with_steps.push(point);
    }

    points_with_steps
  }

  fn get_end(&self) -> Option<Point> {
    self.points.last().cloned()
  }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Clone, Copy)]
struct Point {
  x: i32,
  y: i32,
  steps: i32,
}

fn main() {
  let fp = File::open("./input.txt")
                .expect("Could not find input file");

  let file = BufReader::new(&fp);

  let wires = file.lines()
                  .map(|l| l.expect("Wire invalid").split(",").map(|s| s.parse::<Edge>().unwrap()).collect())
                  .fold(vec![], | mut acc, mut wire: Vec<Edge> | { // Set points for edges
                    let mut start = Point { x: 0, y: 0, steps: 0 };
                    for edge in &mut wire {
                      edge.points = (*edge).get_points(start);
                      start = edge.get_end().unwrap();
                    } 
                    acc.push(wire);
                    acc
                  })
                  .iter()
                  .fold(vec![], | mut acc: Vec<Vec<Point>>, wire: &Vec<Edge> | { // Flatten wires to vector of points
                    let mut w: Vec<Point> = vec![];
                    for edge in wire {
                      w.append(&mut edge.points.clone())
                    }
                    w.sort_by(|a, b| (i32::abs(a.x) + i32::abs(a.y)).cmp(&(i32::abs(b.x) + i32::abs(b.y))));
                    acc.push(w);
                    acc
                  });


  let mut intersections = vec![];
  for point_a in &wires[0 as usize] {
    for point_b in &wires[1 as usize] {
      if point_a.x == point_b.x && point_a.y == point_b.y {
        intersections.push(Point { x: point_a.x, y: point_a.y, steps: point_a.steps + point_b.steps });
      }
    }
  }

  intersections.sort_by(|a, b| (a.steps).cmp(&(b.steps)));

  println!("{:?}", intersections[0]);
}