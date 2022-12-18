use std::fs;

fn main() {
    let input = fs::read_to_string("inputs/3.txt").expect("Problem while reading from file.");
    println!("First: {}", first_part(&input));
    println!("Second: {}", second_part(input));
}

fn first_part(input: &String) -> i32 {
    let lines = input.lines();
    let mut priorities: Vec<i32> = vec![];
    for line in lines {
        let chars: Vec<char> = line.chars().collect();
        let mut bags = chars.chunks(line.len() / 2);
        let first_bag = bags.next().unwrap();
        let second_bag = bags.next().unwrap();
        'outer: for first_item in first_bag {
            for second_item in second_bag {
                if first_item == second_item {
                    if first_item.is_lowercase() {
                        priorities.push(*first_item as i32 - 96)
                    } else {
                        priorities.push(*first_item as i32 - 64 + 26);
                    }
                    break 'outer
                }
            }
        }
    }
    priorities.iter().sum()
}


fn second_part(input: String) -> i32 {
    let lines: Vec<String> = input.lines().map(|l| l.to_string()).collect();
    let groups = lines.chunks(3);
    let mut priorities: Vec<i32> = vec![];
    for group in groups {
        let mut iter = group.iter();
        let first_elf = iter.next().unwrap();
        let second_elf = iter.next().unwrap();
        let third_elf = iter.next().unwrap();
        'outer: for first_item in first_elf.chars() {
            for second_item in second_elf.chars() {
                if first_item == second_item {
                    for third_item in third_elf.chars() {
                        if second_item == third_item {
                            if first_item.is_lowercase() {
                                priorities.push(first_item as i32 - 96)
                            } else {
                                priorities.push(first_item as i32 - 64 + 26);
                            }
                            break 'outer
                        }
                    }
                }
            }
        }
    }
    priorities.iter().sum()
}
