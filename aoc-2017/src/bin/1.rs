use std::fs;

fn main() {
    let input = fs::read_to_string("inputs/1-2.txt").expect("Problem while reading from file.");
    let mut chars = input.chars();
    chars.next_back();
    let values: Vec<u32> = chars.map(|x| x.to_digit(10).unwrap()).collect();

    let mut sum = 0;
    let mut last_value = 0;

    for val in &values {
        if val == &last_value {
            sum += val
        }
        last_value = *val
    }
    if values.last().unwrap() == values.first().unwrap() {
        sum += values.first().unwrap();
    }

    println!("Sum: {}", sum);
}
