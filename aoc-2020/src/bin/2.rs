use std::fs; 

fn main() {
    let input = fs::read_to_string("inputs/1-2.txt").expect("Problem while reading from file.");
    let values: Vec<i32> = input.lines().map(|v| v.parse::<i32>().expect("Failed to parse")).collect();

    'outer: for value1 in &values {
        let pivot1 = value1;
        for value2 in &values {
            let pivot2 = value2;
            for value3 in &values {
                if pivot1 + pivot2 + value3 == 2020 {
                    println!("{} * {} * {} : {}", pivot1, pivot2, value3, pivot1 * pivot2 * value3);
                    break 'outer
                }
            }
        }
    }
}
