use std::fs;

fn main() {
    let input = fs::read_to_string("inputs/9-10.txt").expect("Problem while reading from file.");

    let mut parts = input.split("\n\n");
    let rules: Vec<(i32, i32)> = parts
        .next()
        .unwrap()
        .lines()
        .map(|r| {
            let mut parsed = r.split('|');
            (
                parsed.next().unwrap().parse::<i32>().unwrap(),
                parsed.next().unwrap().parse::<i32>().unwrap(),
            )
        })
        .collect();
    println!("{:?}", rules);

    let updates: Vec<Vec<i32>> = parts
        .next()
        .unwrap()
        .lines()
        .map(|l| {
            l.split(',')
                .map(|u| u.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect();

    for update_line in updates {
        println!("{:?}", update_line);
        let length = update_line.len();
        for i in 0..length {
            let update = update_line[i];
            let relevant_rules: Vec<&(i32, i32)> = rules
                .iter()
                .filter(|x| x.0 == update || x.1 == update)
                .collect();
            let r_length = relevant_rules.len();

            for comparisoni in 0..length {
                let comparison = update_line[comparisoni];
                if comparison == update {
                    continue;
                }
                println!("{} {}", update, comparison);

                for rulei in 0..r_length {
                    let rule = relevant_rules[rulei];
                    println!("{:?}", rule);
                }
            }
        }
    }
}
