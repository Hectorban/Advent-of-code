use std::fs;

fn main() {
  let input = fs::read_to_string("inputs/3-4.txt").expect("Problem while reading from file."); 
  let instructions = input.lines();

  let mut depth = 0;
  let mut horizontal_pos = 0;
  let mut aim = 0;

  for inst in instructions {
    let split_inst: Vec<&str> = inst.split(' ').collect();
    let ammount_of_moves = split_inst[1].parse::<i32>().unwrap();

    match split_inst[0] {
      "forward" => {
        horizontal_pos += ammount_of_moves;
        depth += ammount_of_moves * aim;
      },
      "down" => aim += ammount_of_moves,
      "up" => aim -= ammount_of_moves,
      _ => println!("Not a valid command!")
    }
  }

  println!("Final positon: {}", depth * horizontal_pos)
}