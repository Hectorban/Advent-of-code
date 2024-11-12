use std::fs;

fn main() {
    let input = fs::read_to_string("inputs/3-4.txt").expect("Problem while reading from file.");
    let rows = input.lines();
    println!("{:?}", rows);
    let mut sum = 0;
    for row in rows {
        let columns = row.split('\t').map(|str| str.parse::<u32>().unwrap());
        let max = columns.clone().max().unwrap();
        let min = columns.clone().min().unwrap();
        let diff = max - min;
        sum += diff
    }
    println!("SUM {}", sum);
}
