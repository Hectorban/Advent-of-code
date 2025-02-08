use std::fs;


fn main() {
    let input = fs::read_to_string("inputs/1-2.txt").expect("Problem while reading from file.");
    println!("{}", input);

    let lines = input.lines();

    let mut list_one:Vec<i32> = vec![];
    let mut list_two:Vec<i32> = vec![];

    let mut arrays = vec![1, 2, 3];
    arrays[4] = 5;
 
    for line in lines {
        let mut split = line.split(' ');
        let first_number: i32 = split.next().unwrap().parse().unwrap();
        let second_number: i32 = split.nth(2).unwrap().parse().unwrap();
        list_one.push(first_number);
        list_two.push(second_number);
    }
    
    list_one.sort();
    list_two.sort();

    let mut sum = 0;

    for (a, b) in list_one.into_iter().zip(list_two) {
        let distance = (a - b).abs();
        sum += distance;
    }

    println!("{}", sum);
}
