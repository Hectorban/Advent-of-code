use core::f64;
fn main() {
    let input = "277678";
    //let input  fs::read_to_string("inputs/5-6.txt").expect("Problem while reading from file.");
    let number: i64 = input.lines().next().unwrap().parse().unwrap();

    let upper_limit = get_upper_limit(number);
    let square = upper_limit * upper_limit;

    println!("{} * {} = {}", upper_limit, upper_limit, square);
    println!("{}", upper_limit - 1);
    println!("{}", square - number);
    println!("{}", (upper_limit - 1) - (square - number));
}

fn get_upper_limit(number: i64) -> i64 {
    let fractional_root = (number as f64).sqrt();
    if fractional_root.fract() == 0.0 {
        if (fractional_root as i64 % 2) == 0 {
            return fractional_root as i64 + 1;
        } else {
            return fractional_root as i64;
        }
    }

    let square_root = fractional_root as i64 + 1;
    if (square_root % 2) == 0 {
        square_root + 1
    } else {
        square_root
    }
}
