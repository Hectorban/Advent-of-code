use std::fs;

fn main() {
    let input = fs::read_to_string("inputs/2.txt").expect("Problem while reading from file.");
    println!("First: {}", first_part(&input));
    println!("Second: {}", second_part(input));
}

fn first_part(input: &String) -> i32 {
    let lines = input.lines();
    lines.into_iter().map(|round| {
        let mut chars = round.chars();
        let opponent = chars.next().unwrap();
        let player_obf = chars.nth(1).unwrap();
        let player = convert_player_to_abc(player_obf);
        let play_points = play_to_points(&player);
        let match_points = calc_rps(player, opponent);
        play_points + match_points
    }).sum()
}

fn convert_player_to_abc(play: char) -> char {
    match play {
        'X' => 'A',
        'Y' => 'B',
        'Z' => 'C',
        _ => 'A'
    }
}

fn calc_rps(player: char, opponent: char) -> i32 {
    match player {
        'A' => {
            match opponent {
                'C' => 6, 
                'A' => 3, 
                _ => 0
            }
        },
        'B' => {
            match opponent {
                'A' => 6, 
                'B' => 3, 
                _ => 0
            }
        },
        'C' => {
            match opponent {
                'B' => 6, 
                'C' => 3, 
                _ => 0
            }
        },
        _ => 0
    }
}

fn play_to_points(play: &char) -> i32 {
    match play {
        'A' => 1,
        'B' => 2,
        'C' => 3,
        _ => 0
    }
}

fn second_part(input: String) -> i32 {
    let lines = input.lines();
    lines.into_iter().map(|round| {
        let mut chars = round.chars();
        let opponent = chars.next().unwrap();
        let round_result = chars.nth(1).unwrap();
        let player = calculate_play(round_result, opponent);
        let play_points = play_to_points(&player);
        let match_points = calc_rps(player, opponent);
        play_points + match_points
    }).sum()
}

fn calculate_play(round_result: char, opponent: char) -> char {
    match round_result {
        'X' => {
            match opponent {
                'A' => 'C',
                'B' => 'A',
                'C' => 'B',
                _ => 'Z'
            }
        },
        'Y' => opponent,
        'Z' => {
            match opponent {
                'A' => 'B',
                'B' => 'C',
                'C' => 'A',
                _ => 'Z'
            }
        },
        _ => 'Z'
    }
}

