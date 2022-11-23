use std::fs;

#[derive(Debug)]
enum Orientation {
    Up,
    Right,
    Down,
    Left
}

impl Orientation {
    pub fn rotate_right(self) -> Self {
        match self {
            Orientation::Up => Orientation::Right,
            Orientation::Right => Orientation::Down,
            Orientation::Down => Orientation::Left,
            Orientation::Left => Orientation::Up,
        }
    }

    pub fn rotate_left(self) -> Self {
        match self {
            Orientation::Up => Orientation::Left,
            Orientation::Right => Orientation::Up,
            Orientation::Down => Orientation::Right,
            Orientation::Left => Orientation::Down,
        }
    }
}

fn main() {
    let input = fs::read_to_string("inputs/1-2.txt").unwrap();
    let instructions = input.split(", ");
    let mut north_steps = 0;
    let mut east_steps = 0;
    let mut current_orientation = Orientation::Up;
    for instruction in instructions {
        let mut m_ins = instruction.to_string();
        let turn = m_ins.remove(0);
        let step_r = m_ins.parse::<i32>();
        match step_r {
            Ok(step) => {
                match turn {
                    'L' => current_orientation = current_orientation.rotate_left(),
                    'R' => current_orientation = current_orientation.rotate_right(),
                    _ => panic!("This is not a valid orientation!")
                }
                match current_orientation {
                    Orientation::Up => north_steps += step,
                    Orientation::Right => east_steps += step,
                    Orientation::Down => north_steps -= step,
                    Orientation::Left => east_steps -= step,
                };
            },
            Err(_err) => println!("{}", instruction),
        }
    }

    println!("Last orientation: {:?}", current_orientation);
    println!("North steps {}", north_steps);
    println!("East steps {}", east_steps);
    println!("Answer {}", -east_steps + north_steps);
}
