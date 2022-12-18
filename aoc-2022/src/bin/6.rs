use std::{fs, collections::HashSet};

fn main() {
    let input = fs::read_to_string("inputs/6.txt").expect("Problem while reading from file.");
    println!("First: {}", first_part(&input));
    println!("Second: {}", second_part(input));
}

fn first_part(input: &String) -> String {
    let i_stream: Vec<char> = input.chars().collect();
    (input.chars().enumerate().find(|(i, character)|{
        if *i > 3 {
            let last_4 = vec![i_stream[i-1], i_stream[i-2], i_stream[i-3], *character];
            let mut uniq = HashSet::new();
            last_4.into_iter().all(move |x| uniq.insert(x))
        } else { false }
    }).unwrap().0 + 1).to_string()
}

fn second_part(input: String) -> String {
    let i_stream: Vec<char> = input.chars().collect();
    (input.chars().enumerate().find(|(i, character)|{
        if *i > 13 {
            let mut last_14: Vec<char> = vec![];
            last_14.push(*character);
            for index in 1..=13 { last_14.push(i_stream[i-index]) }
            let mut uniq = HashSet::new();
            last_14.into_iter().all(move |x| uniq.insert(x))
        } else { false }
    }).unwrap().0 + 1).to_string()
}
