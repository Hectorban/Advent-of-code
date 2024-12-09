use std::fs;

fn main() {
    let input = fs::read_to_string("inputs/1-2.txt").expect("Problem while reading from file.");

    let lines = input.lines();

    let mut list_one:Vec<usize> = vec![];
    let mut list_two:Vec<usize> = vec![];

    for line in lines {
        let mut split = line.split(' ');
        let first_number: usize= split.next().unwrap().parse().unwrap();
        let second_number: usize = split.nth(2).unwrap().parse().unwrap();
        list_one.push(first_number);
        list_two.push(second_number);
    }

    let mut sum = 0;
    for number in &list_one {
        let appereances = list_two.clone().into_iter().filter(|number_two| number_two == number).count();
        sum += number * appereances;
    }

    println!("{}", sum);
}
