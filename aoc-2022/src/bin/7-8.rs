use std::{fs, ops::Range};

fn main() {
    let input = fs::read_to_string("inputs/7-8.txt").expect("Problem while reading from file.");
    println!("First: {}", first_part(&input));
    println!("Second: {}", second_part(input));
}

fn first_part(input: &String) -> i32 {
    let lines = input.lines();
    let result = lines.into_iter().map(|line| {
        let mut split = line.split(",");
        let first_elf = gen_range(split.next().unwrap());
        let second_elf = gen_range(split.next().unwrap());
        let second_to_first = first_elf.clone().into_iter().all(|item| second_elf.contains(&item));
        let first_to_second = second_elf.clone().into_iter().all(|item| first_elf.contains(&item));
        if first_to_second {
            println!("{:?} contains {:?}", first_elf, second_elf);
        }
        if second_to_first {
            println!("{:?} contains {:?}", second_elf, first_elf);
        }
        first_to_second | second_to_first
    }).fold(0, |acc, x| {
        if x {
            acc + 1
        } else {
            acc + 0
        }
    });
    println!("{}", result);
    0
}

fn gen_range(range: &str) -> Range<i32> {
    let mut split = range.split("-");
    let start: i32 = split.next().unwrap().parse().unwrap();
    let end: i32 = split.next().unwrap().parse().unwrap();
    start..end
}

fn second_part(input: String) -> i32 {
    0
}
    
