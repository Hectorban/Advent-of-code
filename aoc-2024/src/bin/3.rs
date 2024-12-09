use std::fs;

fn main() {
    let input = fs::read_to_string("inputs/3-4.txt").expect("Problem while reading from file.");
    let response = input
        .lines()
        .map(|report| {
            report
                .split(' ')
                .map(|level| level.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .fold(0, |acc, report| {
            if has_safe_direction(&report) && has_safe_values(&report) {
                acc + 1
            } else {
                acc
            }
        });

    println!("{}", response);
}

fn has_safe_values(levels: &Vec<i32>) -> bool {
    let mut last_level = levels[0];
    let length = levels.len();

    for level in &levels[1..length] {
        if (last_level - *level).abs() > 3 {
            return false;
        }
        last_level = *level;
    }

    true
}

fn has_safe_direction(levels: &Vec<i32>) -> bool {
    let mut last_level = levels[0];
    let length = levels.len();
    let mut direction_acc: i32 = 0;

    for level in &levels[1..length] {
        if last_level == *level {
            return false;
        } else if last_level <= *level {
            direction_acc += 1;
        } else {
            direction_acc -= 1;
        }
        last_level = *level;
    }

    if direction_acc.abs() != (length - 1).try_into().unwrap() {
        return false;
    }

    true
}
