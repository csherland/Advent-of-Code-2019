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
}
