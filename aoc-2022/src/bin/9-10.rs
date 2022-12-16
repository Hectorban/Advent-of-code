use std::fs;

fn main() {
    let input = fs::read_to_string("inputs/9-10.txt").expect("Problem while reading from file.");
    println!("First: {}", first_part(&input));
    println!("Second: {}", second_part(input));
}

fn first_part(input: &String) -> String {
    let lines = input.lines();
    let mut boxes: Vec<&str> = lines.collect();
    let mut instructions = boxes.split_off(8);
    instructions.remove(0);
    instructions.remove(0);
    let mut matrix = create_box_matrix(boxes);
    for instruction in instructions {
        let (ammount, from, to) = parse_instruction(instruction);
        for _ in 0..ammount {
            let moved_box = matrix[from-1].pop().unwrap();
            matrix[to-1].push(moved_box.clone());
        }
    }
    matrix.into_iter().fold("".to_string(),|acc, a| format!("{}{}", acc, a.last().unwrap().to_string()))
}

fn create_box_matrix(boxes: Vec<&str>) -> Vec<Vec<String>> {
    let mut matrix: Vec<Vec<String>> = vec![];
    let box_stack_indexes = vec![1, 5, 9, 13, 17, 21, 25, 29, 33];
    for box_stack in box_stack_indexes {
        let mut box_stack_vector: Vec<String> = vec![];
        for box_line in &boxes {
            let box_at_index = box_line.chars().nth(box_stack).unwrap();
            if box_at_index != ' ' {
                box_stack_vector.push(box_at_index.to_string());
            }
        }
        matrix.push(box_stack_vector.into_iter().rev().collect());
    }
    matrix
}

fn parse_instruction(instruction: &str) -> (usize, usize, usize) {
    let split: Vec<&str> = instruction.split(' ').collect();
    let ammount = split[1].parse::<usize>().unwrap();
    let from = split[3].parse::<usize>().unwrap();
    let to = split[5].parse::<usize>().unwrap();
    (ammount, from, to)
}

fn second_part(input: String) -> String {
    let lines = input.lines();
    let mut boxes: Vec<&str> = lines.collect();
    let mut instructions = boxes.split_off(8);
    instructions.remove(0);
    instructions.remove(0);
    let mut matrix = create_box_matrix(boxes);
    for instruction in instructions {
        let (ammount, from, to) = parse_instruction(instruction);
        let final_length = matrix[from-1].len().saturating_sub(ammount);
        let mut tail_box = matrix[from-1].split_off(final_length);
        matrix[to-1].append(&mut tail_box);
    }
    matrix.into_iter().fold("".to_string(),|acc, a| format!("{}{}", acc, a.last().unwrap().to_string()))
}
