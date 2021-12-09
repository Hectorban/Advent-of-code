use std::fs;

fn main() {
  let input = fs::read_to_string("inputs/14-15.txt").expect("Problem reading file");
  println!("{}", input)
}