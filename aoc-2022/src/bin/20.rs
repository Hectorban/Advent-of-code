use std::fs;

fn main() {
    let input = fs::read_to_string("inputs/20.txt").expect("Problem while reading from file.");
    println!("First: {}", first_part(&input));
    println!("Second: {}", second_part(input));
}

fn first_part(input: &String) -> String {
    "Not solved".to_string()
}

fn second_part(input: String) -> String {
    "Not solved".to_string()
}
