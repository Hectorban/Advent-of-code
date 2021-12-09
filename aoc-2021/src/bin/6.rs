use std::fs;
use std::cmp::Ordering;

fn main() {
    let oxygen_generator = calculate_life_system_rating('1', '0');
    let co2_scrubber = calculate_life_system_rating('0', '1');
    println!("Oxygen generator: {}", oxygen_generator);
    println!("Co2 scrubber: {}", co2_scrubber);
    println!("Life support rating: {}", oxygen_generator * co2_scrubber)
} 

fn calculate_life_system_rating(acc_1: char, acc_2: char) -> isize {
  let input = fs::read_to_string("inputs/5-6.txt").expect("Problem while reading from file."); 
  let diag_chars = input.lines().nth(0).unwrap().chars();
  
  let mut life_system: Vec<&str> = input.lines().collect();

  for (index, _character) in diag_chars.enumerate() {
    if life_system.clone().into_iter().count() == 1 { break; }
    let arrays_slice: Vec<usize> = life_system.clone().into_iter().map(|d| d.chars().nth(index).unwrap().to_digit(10).unwrap() as usize).collect();
    let ammount_of_1: usize = arrays_slice.clone().into_iter().sum();
    let ammount_of_0: usize = arrays_slice.clone().into_iter().fold(0, |acc, x| acc + if x == 0 {1} else {0});
    match ammount_of_1.cmp(&ammount_of_0) {
      Ordering::Equal => life_system = life_system.into_iter().filter(|&x| x.chars().nth(index).unwrap() == acc_1).collect(),
      Ordering::Greater => life_system = life_system.into_iter().filter(|&x| x.chars().nth(index).unwrap() == acc_1).collect(),
      Ordering::Less => life_system = life_system.into_iter().filter(|&x| x.chars().nth(index).unwrap() == acc_2).collect()
    }
  }
  isize::from_str_radix(&life_system[0], 2).unwrap()
}
