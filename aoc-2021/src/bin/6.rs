use std::fs;

fn main() {
  let input = fs::read_to_string("inputs/5-6.txt").expect("Problem while reading from file."); 
  print!("{}", input)
}