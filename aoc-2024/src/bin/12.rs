use std::collections::HashSet;
use std::usize;

const INPUT: &str = include_str!("../../inputs/11-12.txt");

const DIRECTIONS: [(isize, isize); 4] = [
    (0, -1), // Up
    (1, 0),  // Right
    (0, 1),  // Down
    (-1, 0), // Left
];

fn main() {
    use std::time::Instant;
    let now = Instant::now();

    let mut starting_position = (0, 0);
    let mut text: Vec<Vec<char>> = INPUT
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect();

    for (j, line) in text.iter().enumerate() {
        for (i, &char) in line.iter().enumerate() {
            if char == '^' {
                starting_position = (i, j);
            }
        }
    }

    let rows = text.len();
    let cols = text[0].len();
    let mut sum = 0;

    for row in 0..rows {
        for col in 0..cols {
            if text[row][col] == '#' {
                continue;
            }
            // Avoid copies
            let original_char = text[row][col];
            text[row][col] = '#';
            if is_in_endless_loop(&text, starting_position, cols, rows) {
                sum += 1;
            }
            text[row][col] = original_char;
        }
    }

    println!("{}", sum);
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}

#[inline(always)]
fn is_in_endless_loop(
    text: &[Vec<char>],
    starting_position: (usize, usize),
    cols: usize,
    rows: usize,
) -> bool {
    let mut visited_states: HashSet<((usize, usize), usize)> = HashSet::new();
    let mut current_position = starting_position;
    let mut current_direction_i = 0;

    loop {
        let nx = current_position.0 as isize + DIRECTIONS[current_direction_i].0;
        let ny = current_position.1 as isize + DIRECTIONS[current_direction_i].1;

        if nx < 0 || ny < 0 || nx >= cols as isize || ny >= rows as isize {
            return false;
        }

        let next_position = text[ny as usize][nx as usize];

        if next_position == '#' {
            current_direction_i = (current_direction_i + 1) % 4;
        } else {
            current_position = (nx as usize, ny as usize);

            if !visited_states.insert((current_position, current_direction_i)) {
                return true;
            }
        }
    }
}
