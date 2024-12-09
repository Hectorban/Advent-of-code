use std::fs;

const DIRECTIONS: [(isize, isize); 4] = [
    (0, -1), // Up
    (1, 0),  // Right
    (0, 1),  // Down
    (-1, 0), // Left
];

fn main() {
    let input = fs::read_to_string("inputs/11-12.txt").expect("Problem while reading from file.");

    let mut current_position = (0, 0);
    let mut text: Vec<Vec<char>> = input
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect();

    for (j, line) in text.clone().into_iter().enumerate() {
        for (i, char) in line.into_iter().enumerate() {
            if char == '^' {
                current_position = (i, j);
            }
        }
    }

    let mut current_direction_i = 0;
    let cols = text.len();
    let rows = text[0].len();

    loop {
        let nx = current_position.0 as isize + DIRECTIONS[current_direction_i].0;
        let ny = current_position.1 as isize + DIRECTIONS[current_direction_i].1;

        if nx < 0 || ny < 0 || nx >= rows as isize || ny >= cols as isize {
            break;
        }

        let next_position = text[ny as usize][nx as usize];

        if next_position == '#' {
            if current_direction_i == 3 {
                current_direction_i = 0;
                continue;
            }
            current_direction_i += 1;
        } else {
            current_position = (nx as usize, ny as usize);
            text[current_position.1][current_position.0] = 'x';
        }
    }

    let sum = text.into_iter().fold(1, |acc, l| {
        acc + l
            .into_iter()
            .fold(0, |acc, c| if c == 'x' { acc + 1 } else { acc })
    });

    println!("{}", sum);
}
