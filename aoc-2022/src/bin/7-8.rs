use std::{fs, collections::HashSet};

fn main() {
    let input = fs::read_to_string("inputs/7-8.txt").expect("Problem while reading from file.");
    println!("First: {}", first_part(&input));
    println!("Second: {}", second_part(input));
}

fn first_part(input: &String) -> usize {
    let lines = input.lines();
    lines.into_iter().filter(|line| {
        let mut split = line.split(",");
        let first = gen_range(split.next().unwrap());
        let second = gen_range(split.next().unwrap());
        second.iter().all(|i| first.contains(i)) || first.iter().all(|i| second.contains(i))
    }).collect::<Vec<&str>>().len()
}

fn gen_range(range: &str) -> HashSet<i32> {
    let mut split = range.split("-");
    let start: i32 = split.next().unwrap().parse().unwrap();
    let end: i32 = split.next().unwrap().parse().unwrap();
    std::ops::RangeInclusive::new(start, end).collect()
}

fn second_part(input: String) -> usize {
    let lines = input.lines();
    lines.into_iter().filter(|line| {
        let mut split = line.split(",");
        let first_elf = gen_range(split.next().unwrap());
        let second_elf = gen_range(split.next().unwrap());
        !first_elf.intersection(&second_elf).collect::<Vec<&i32>>().is_empty()
    }).collect::<Vec<&str>>().len()
}
    
