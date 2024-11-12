use std::fs;

fn main() {
    let input = fs::read_to_string("inputs/3-4.txt").expect("Problem while reading from file.");
    let rows = input.lines();
    let mut sum = 0;

    for row in rows {
        println!("{}", row);
        let columns = row.split('\t').map(|str| str.parse::<u32>().unwrap());

        for current_column in columns.clone() {
            for checked_column in columns.clone() {
                if current_column <= checked_column {
                    continue;
                }
                if (current_column % checked_column) == 0 {
                    sum += current_column / checked_column
                }
            }
        }
    }
    println!("SUM {}", sum);
}
