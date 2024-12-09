use std::{char, collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string("inputs/15-16.txt").expect("Problem while reading from file.");
    let mut antennas: HashMap<char, Vec<(isize, isize)>> = HashMap::new();

    let mut map: Vec<Vec<char>> = input
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect();

    let rows = map.len() as isize;
    let cols = map[0].len() as isize;

    for row in 0..rows {
        for col in 0..cols {
            let char = map[row as usize][col as usize];
            if char != '.' {
                if let Some(vec) = antennas.get_mut(&char) {
                    vec.push((row, col))
                } else {
                    antennas.insert(char, vec![(row, col)]);
                }
            }
        }
    }

    for char_coords in antennas {
        let length = char_coords.1.len();
        for origin in 0..length {
            let origin_coords = char_coords.1[origin];
            for external in 0..length {
                let external_coords = char_coords.1[external];
                if external_coords == origin_coords {
                    continue;
                }

                let dx = origin_coords.1 - external_coords.1;
                let dy = origin_coords.0 - external_coords.0;

                for k in 0..rows {
                    let extended1 = (origin_coords.0 + (dy * k), origin_coords.1 + (dx * k));
                    let extended2 = (external_coords.0 - (dy * k), external_coords.1 - (dx * k));

                    if !(extended1.1 >= rows
                        || extended1.0 < 0
                        || extended1.0 >= cols
                        || extended1.1 < 0)
                    {
                        map[extended1.0 as usize][extended1.1 as usize] = '#';
                    } else if !(extended2.1 >= rows
                        || extended2.1 < 0
                        || extended2.0 >= cols
                        || extended2.0 < 0)
                    {
                        map[extended2.0 as usize][extended2.1 as usize] = '#';
                    }
                }
            }
        }
    }

    let mut sum = 0;

    for row in map {
        for char in row {
            print!("{}", char);
            if char == '#' {
                sum += 1;
            }
        }
        println!();
    }

    println!("{}", sum);
}
