use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Problem while reading from file."); 
    let first_char = first_char_in_basement(input);
    println!("The first char that entered the bastement is: {}", first_char);
}

fn first_char_in_basement(instructions: String) -> usize {
    let mut floor_level: i32 = 0;
    let mut index_of_char: usize = 0;

    for (iterator, caracter) in instructions.chars().enumerate() {
        println!("Floor level is: {}", floor_level);
        match caracter {
            '(' => floor_level += 1,
            ')' => floor_level -= 1,
            _ => print!("nel") 
        };
        if floor_level < 0 {
            index_of_char = iterator + 1;
            break
        }
    }
    index_of_char
}