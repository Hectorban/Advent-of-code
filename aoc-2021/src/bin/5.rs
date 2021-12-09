use std::fs;
use std::cmp::Ordering;

fn main() {
  let input = fs::read_to_string("inputs/5-6.txt").expect("Problem while reading from file."); 
  let diag_chars = input.lines().nth(0).unwrap().chars();
  
  let total_diagnostics  = &input.lines().count();
  let mut gamma_rate = String::from("");
  let mut epsilon_rate = String::from("");
  
  for (index, _character) in diag_chars.enumerate() {
    let diagnostics: usize = input
                        .lines()
                        .map(|d| d.chars().nth(index).unwrap().to_digit(10).unwrap() as usize)
                        .sum();

    let common_factor = total_diagnostics/2;
    match diagnostics.cmp(&common_factor) {
      Ordering::Equal => println!("Equal???"),
      Ordering::Greater => {
        gamma_rate.push('1');
        epsilon_rate.push('0');
      },
      Ordering::Less => {
        gamma_rate.push('0');
        epsilon_rate.push('1');
      }        
    }
  }

  let bin_gamma_rate = isize::from_str_radix(&gamma_rate, 2).unwrap();
  let bin_epsilon_rate = isize::from_str_radix(&epsilon_rate, 2).unwrap();
  println!("The response is {}", bin_gamma_rate * bin_epsilon_rate);
} 
