use std::fs;

fn main() {
    // 900
    // 2*l*w + 2*w*h + 2*h*l
    let input = fs::read_to_string("input.txt").expect("Problem while reading from file."); 
    let beautified_input = input.replace("\n", ",");
    let list_of_presents = beautified_input.split(",").collect::<Vec<&str>>();

    let mut total_area_of_paper = 0;

    for m in list_of_presents {
        let array_of_measurements = m.trim().split("x").collect::<Vec<&str>>();
        let meas: Vec<i32> = array_of_measurements.into_iter().map(|x| x.parse::<i32>().unwrap()).collect();

        let tuple_of_meas = (meas[0], meas[1], meas[2]);
        let present_area = calculate_wrapping_area(tuple_of_meas);
        total_area_of_paper += present_area
    }

    println!("The area need for the gift is: {}", total_area_of_paper)
}

fn calculate_wrapping_area(d: (i32, i32, i32)) -> i32 {
    let array_of_sides = [d.0*d.1, d.1*d.2, d.0*d.2];
    let smallest_side = array_of_sides.iter().min().unwrap();

    array_of_sides[0] + array_of_sides[1] + array_of_sides[2] + smallest_side
}
