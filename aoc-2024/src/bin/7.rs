use std::fs;

const DIRECTIONS: [(isize, isize); 8] = [
    (-1, 0),
    (1, 0),
    (0, -1),
    (0, 1),
    (-1, -1),
    (-1, 1),
    (1, -1),
    (1, 1),
];

fn main() {
    let input = fs::read_to_string("inputs/7-8.txt").expect("Problem while reading from file.");

    let text: Vec<&str> = input.lines().collect();

    let rows = text.len();
    let cols = text[0].len();
    let mut sum = 0;

    for i in 0..rows {
        for j in 0..cols {
            if text[i].as_bytes()[j] == b'X' {
                for (dx, dy) in DIRECTIONS {
                    if find_xmas(&text, i, j, dx, dy, rows, cols) {
                        println!(
                            "'XMAS' found starting at ({}, {}) in direction ({}, {})",
                            i, j, dx, dy
                        );

                        sum += 1
                    }
                }
            }
        }
    }

    println!("{}", sum);
}

fn find_xmas(
    text: &[&str],
    x: usize,
    y: usize,
    dx: isize,
    dy: isize,
    rows: usize,
    cols: usize,
) -> bool {
    let target = b"XMAS";
    for k in 0..4 {
        let nx = x as isize + k * dx;
        let ny = y as isize + k * dy;
        if nx < 0 || ny < 0 || nx >= rows as isize || ny >= cols as isize {
            return false;
        }
        if text[nx as usize].as_bytes()[ny as usize] != target[k as usize] {
            return false;
        }
    }
    true
}
