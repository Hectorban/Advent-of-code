use std::fs;

fn main() {
  let input = fs::read_to_string("inputs/1-2.txt").expect("Problem while reading from file."); 
  
  let mut last_window = 0;
  let mut ammount_of_increases: i32 = 0;
  let input_lines = input.lines();

  for(iter, value) in input_lines.enumerate() {
    let mut mut_input_lines = input.lines();
    
    let a = value.parse::<i32>().unwrap();
    let b = mut_input_lines.nth(iter + 1).unwrap().parse::<i32>().unwrap();
    let c = mut_input_lines.nth(iter + 2).unwrap().parse::<i32>().unwrap();

    let sliding_window = a + b + c;
    if sliding_window > last_window { ammount_of_increases += 1 }
    last_window = sliding_window;
    
    if iter == mut_input_lines.count() - 3 { break };
  }
  println!("ammount of increases {}", ammount_of_increases - 1);
}