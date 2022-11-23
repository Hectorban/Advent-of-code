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

#[derive(Debug, Clone, Copy, PartialEq)]
struct Coordinate {
    x: i16,
    y: i16
}

struct Map {
    coord: Coordinate,
    memo: Vec<Coordinate>
}

impl Map {
    pub fn move_up(&mut self) {
        self.coord.y += 1;
    }

    pub fn move_down(&mut self) {
        self.coord.y -= 1;
    }

    pub fn move_right(&mut self) {
        self.coord.x += 1;
    }

    pub fn move_left(&mut self) {
        self.coord.x -= 1;
    }
}


fn main() {
    //let test_input = "R8, R4, R4, R8";
    let input = fs::read_to_string("inputs/1-2.txt").unwrap();
    let instructions = input.split(", ");
    let mut map = Map { coord: Coordinate { x: 0, y: 0 }, memo: vec![Coordinate { x: 0, y: 0 }] };
    let mut current_orientation = Orientation::Up;
    'outer: for instruction in instructions {
        let mut m_ins = instruction.to_string();
        let turn = m_ins.remove(0);
        let step_r = m_ins.parse::<i16>();
        match step_r {
            Ok(steps) => {
                match turn {
                    'L' => current_orientation = current_orientation.rotate_left(),
                    'R' => current_orientation = current_orientation.rotate_right(),
                    _ => panic!("This is not a valid orientation!")
                }
                for _step in 0..steps {
                    match current_orientation {
                        Orientation::Up => map.move_up(),
                        Orientation::Right => map.move_right(),
                        Orientation::Down => map.move_down(),
                        Orientation::Left => map.move_left(),
                    };
                    let found = map.memo.contains(&map.coord);
                    if found {
                        println!("{}, {}", map.coord.x, map.coord.y);
                        println!("Response: {}", -map.coord.x + map.coord.y);
                        break 'outer;
                    }
                    map.memo.push(map.coord);
                }
            },
            Err(_err) => println!("{}", instruction),
        }
    }
}
