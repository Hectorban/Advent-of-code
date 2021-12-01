use std::fs;

fn main() {
  let input = fs::read_to_string("inputs/1-2.txt").expect("Problem while reading from file."); 
  let mut last_measurement = 0;
  let mut number_of_increases = 0;

  for line in input.lines() {
    let parsed_line = line.parse::<i32>().unwrap();
    if parsed_line > last_measurement { number_of_increases += 1 }
    last_measurement = parsed_line;
  } 
  
  print!("{}", number_of_increases - 1)
}