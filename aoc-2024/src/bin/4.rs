use std::fs;

fn main() {
    let input = fs::read_to_string("inputs/3-4.txt").expect("Problem while reading from file.");
    let reports = input.lines();

    let mut sum = 0;

    for report in reports {
        let levels: Vec<i32> = report
            .split(' ')
            .map(|l| l.parse::<i32>().unwrap())
            .collect();

        if !safe_direction(&levels) || !has_safe_values(&levels) {
            let mut safe = false;

            for (i, _) in levels.clone().into_iter().enumerate() {
                let mut l_clone = levels.clone();
                l_clone.remove(i);
                if safe_direction(&l_clone) && has_safe_values(&l_clone) {
                    safe = true;
                    break;
                }
            }

            if !safe {
                continue;
            }
        }

        sum += 1;
    }

    println!("{}", sum);
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

fn safe_direction(levels: &Vec<i32>) -> bool {
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
