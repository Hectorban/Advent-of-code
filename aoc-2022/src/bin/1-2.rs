use std::fs;

fn main() {
    let input = fs::read_to_string("inputs/1-2.txt").expect("Problem while reading from file.");
    println!("First: {}", first_part(&input));
    println!("Second: {}", second_part(input))
}

fn first_part(input: &String) -> i32 {
    let lines = input.lines();
    let mut elf_calories: Vec<i32> = vec![];
    let mut curr_elf_count = 0;
    for line in lines {
        if line.is_empty() { 
            elf_calories.push(curr_elf_count);
            curr_elf_count = 0;
            continue 
        }
        let calories = line.parse::<i32>().unwrap();
        curr_elf_count += calories
    }
    *elf_calories.iter().max().unwrap()
}

fn second_part(input: String) -> i32 {
    let lines = input.lines();
    let mut elf_calories: Vec<i32> = vec![];
    let mut curr_elf_count = 0;
    for line in lines {
        if line.is_empty() { 
            elf_calories.push(curr_elf_count);
            curr_elf_count = 0;
            continue 
        }
        let calories = line.parse::<i32>().unwrap();
        curr_elf_count += calories
    }
    elf_calories.sort_by(|a, b| b.cmp(a));
    elf_calories.truncate(3);
    elf_calories.into_iter().sum::<i32>()
}
