use std::fs;

fn main() {
    let input = fs::read_to_string("inputs/7-8.txt").expect("Problem while reading from file.");

    let text: Vec<&str> = input.lines().collect();
    let grid: Vec<Vec<char>> = text.iter().map(|&line| line.chars().collect()).collect();

    let permutations = [
        [
            ['M', '.', 'S'],
            ['.', 'A', '.'],
            ['M', '.', 'S'],
        ],
        [
            ['S', '.', 'M'],
            ['.', 'A', '.'],
            ['S', '.', 'M'],
        ],
        [
            ['M', '.', 'M'],
            ['.', 'A', '.'],
            ['S', '.', 'S'],
        ],
        [
            ['S', '.', 'S'],
            ['.', 'A', '.'],
            ['M', '.', 'M'],
        ],
    ];

    let mut sum = 0;

    for y in 1..grid.len() - 1 {
        for x in 1..grid[y].len() - 1 {
            if grid[y][x] == 'A' {
                for perm in &permutations {
                    if matches_pattern(&grid, x, y, perm) {
                        sum += 1;
                        break;
                    }
                }
            }
        }
    }

    println!("{}", sum)
}

fn matches_pattern(grid: &[Vec<char>], x: usize, y: usize, pattern: &[[char; 3]; 3]) -> bool {
    for dy in 0..3 {
        for dx in 0..3 {
            let grid_char = grid[y + dy - 1][x + dx - 1];
            let pattern_char = pattern[dy][dx];
            if pattern_char != '.' && pattern_char != grid_char {
                return false;
            }
        }
    }
    true
}
