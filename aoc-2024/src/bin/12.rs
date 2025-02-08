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

    let mut current_position = (0, 0);
    let mut starting_position = (0, 0);
    let text: Vec<Vec<char>> = INPUT
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect();

    for (j, line) in text.iter().enumerate() {
        for (i, &char) in line.iter().enumerate() {
            if char == '^' {
                current_position = (i, j);
                starting_position = (i, j);
            }
        }
    }

    let mut current_direction_i = 0;
    let cols = text.len();
    let rows = text[0].len();
    let mut sum = 0;
    let mut obstacles_checked: HashSet<(isize, isize)> = HashSet::new();

    loop {
        let nx = current_position.0 as isize + DIRECTIONS[current_direction_i].0;
        let ny = current_position.1 as isize + DIRECTIONS[current_direction_i].1;

        if nx < 0 || ny < 0 || nx >= cols as isize || ny >= rows as isize {
            break;
        }

        let next_position = text[ny as usize][nx as usize];

        if next_position == '#' {
            current_direction_i = (current_direction_i + 1) % 4;
        } else {
            let mut alternate_text = text.clone();
            alternate_text[ny as usize][nx as usize] = '0';
            if is_in_endless_loop(&alternate_text, starting_position, cols, rows)
                && obstacles_checked.insert((nx, ny))
            {
                sum += 1;
            }
            current_position = (nx as usize, ny as usize);
        }
    }

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
    println!("Result: {}", sum);
}

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

        if next_position == '#' || next_position == '0' {
            current_direction_i = (current_direction_i + 1) % 4;
        } else {
            current_position = (nx as usize, ny as usize);

            if !visited_states.insert((current_position, current_direction_i)) {
                return true;
            }
        }
    }
}
