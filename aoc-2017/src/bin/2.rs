#![feature(iter_advance_by)]
use std::fs;

fn main() {
    //let input = "1212";
    let input = fs::read_to_string("inputs/1-2.txt").expect("Problem while reading from file.");
    let mut chars = input.chars();
    chars.next_back();
    let values: Vec<u32> = chars.map(|x| x.to_digit(10).unwrap()).collect();

    let length = values.len();
    let half_length = length / 2;
    let mut cycle_iterator = values.clone().into_iter().cycle();
    let mut sum = 0;
    cycle_iterator.advance_by(half_length).unwrap();

    for current_number in values {
        let halfway_number = cycle_iterator.next().unwrap();
        if current_number == halfway_number {
            sum += current_number
        }
    }

    println!("Sum: {}", sum);
}
