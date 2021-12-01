use std::fs;

fn main() {
    let input = fs::read_to_string("input/1-2.txt").expect("Problem while reading from file."); 
    let floor_level = floor_resolver(input);    
    println!("The floor level is: {}", floor_level);
}

fn floor_resolver(instructions: String) -> i32 {
    let mut floor_level: i32 = 0;

    for character in instructions.chars() {
        match character {
            '(' => floor_level += 1,
            ')' => floor_level -= 1,
            _ => print!("nel") 
        };
    }
    floor_level
}