use std::fs;
use std::collections::HashSet;

fn main() {
    let input = fs::read_to_string("inputs/9-10.txt").expect("Problem while reading from file.");

    let rules: HashSet<(u8, u8)> = input
        .lines()
        .take_while(|line| !line.is_empty())
        .filter_map(|line| {
            line.split_once('|')
                .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
        })
        .collect();

    let updates = updates(&input, rules.len() + 1);

    let result: u16 = updates
        .filter_map(|update| {
            update
                .is_sorted_by(|a, b| !rules.contains(&(*b, *a)))
                .then_some(u16::from(update[update.len() / 2]))
        })
        .sum();

    println!("{}", result);
}

fn updates(input: &str, skip_len: usize) -> impl Iterator<Item = Vec<u8>> + '_ {
    input.lines().skip(skip_len).map(|update| {
        update
            .split(',')
            .filter_map(|num| num.parse().ok())
            .collect()
    })
}
