use rustc_hash::FxHashSet;
use std::time::Instant;

const INPUT: &str = include_str!("../../inputs/11-12.txt");

const DIRECTIONS: [(i16, i16); 4] = [
    (0, -1), // Up
    (1, 0),  // Right
    (0, 1),  // Down
    (-1, 0), // Left
];

fn main() {
    let now = Instant::now();

    let mut current_position: (i16, i16) = (0, 0);
    let mut starting_position: (i16, i16) = (0, 0);
    let text: Vec<Vec<char>> = INPUT
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect();

    for (j, line) in text.iter().enumerate() {
        for (i, &char) in line.iter().enumerate() {
            if char == '^' {
                current_position = (i as i16, j as i16);
                starting_position = (i as i16, j as i16);
            }
        }
    }

    let mut current_direction_i = 0;
    let cols: i16 = text.len() as i16;
    let rows: i16 = text[0].len() as i16;
    let mut sum = 0;
    let mut obstacles_checked: FxHashSet<(i16, i16)> = FxHashSet::default();

    loop {
        let nx = current_position.0 + DIRECTIONS[current_direction_i].0;
        let ny = current_position.1 + DIRECTIONS[current_direction_i].1;

        if nx < 0 || ny < 0 || nx >= cols || ny >= rows {
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
            current_position = (nx, ny);
        }
    }

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
    println!("Result: {}", sum);
}

fn is_in_endless_loop(
    text: &[Vec<char>],
    starting_position: (i16, i16),
    cols: i16,
    rows: i16,
) -> bool {
    let mut visited_states: FxHashSet<((i16, i16), usize)> = FxHashSet::default();
    let mut current_position = starting_position;
    let mut current_direction_i = 0;

    loop {
        let nx = current_position.0 + DIRECTIONS[current_direction_i].0;
        let ny = current_position.1 + DIRECTIONS[current_direction_i].1;

        if nx < 0 || ny < 0 || nx >= cols || ny >= rows {
            return false;
        }

        let next_position = text[ny as usize][nx as usize];

        if next_position == '#' || next_position == '0' {
            current_direction_i = (current_direction_i + 1) % 4;
        } else {
            current_position = (nx, ny);

            if !visited_states.insert((current_position, current_direction_i)) {
                return true;
            }
        }
    }
}
