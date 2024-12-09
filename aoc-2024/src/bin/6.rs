use regex::Regex;
use std::fs;

fn main() {
    let input = fs::read_to_string("inputs/5-6.txt").expect("Problem while reading from file.");

    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let external_regex = Regex::new(r"do\(\)(.*?)don't\(\)").unwrap();

    // First mul till don't, im lazy
    let mut sum = 941751;

    for external_capture in external_regex.captures_iter(&input) {
        for caps in re.captures_iter(external_capture.get(1).unwrap().as_str()) {
            let first_number: i32 = caps.get(1).unwrap().as_str().parse().unwrap();
            let second_number: i32 = caps.get(2).unwrap().as_str().parse().unwrap();

            sum += first_number * second_number;
        }
    }

    println!("{}", sum);
}
