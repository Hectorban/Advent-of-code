use std::fs;

fn main() {
    let input = fs::read_to_string("inputs/1-2.txt").expect("Problem while reading from file.");
    let values: Vec<i32> = input.lines().map(|v| v.parse::<i32>().expect("Failed to parse")).collect();

    'outer: for value in &values {
        let pivot = value;
        for value in &values {
           if pivot + value == 2020 {
                println!("{} * {} : {}", pivot, value, pivot * value);
                break 'outer
            } 
        }
    }
}
