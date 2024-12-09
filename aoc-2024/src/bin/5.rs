use regex::Regex;
use std::fs;

fn main() {
    let input = fs::read_to_string("inputs/5-6.txt").expect("Problem while reading from file.");

    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let mut sum = 0;

    for caps in re.captures_iter(&input) {
        let first_number: i32 = caps.get(1).unwrap().as_str().parse().unwrap();
        let second_number: i32 = caps.get(2).unwrap().as_str().parse().unwrap();

        sum += first_number * second_number;
    }

    println!("{}", sum);
}
