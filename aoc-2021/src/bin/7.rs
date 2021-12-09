use std::fs;

#[derive(Debug)]
struct BingoNumber {
  value: i8,
  is_marked: bool
}

fn main(){
  let input = fs::read_to_string("inputs/7-8.txt").expect("Problem while reading from file."); 
  let mut input_lines: Vec<&str> = input.lines().collect();
  let bingo_inputs = input_lines.remove(0);
  for bingo_board in input_lines.chunks(6) {
    let clean_bingo_board: Vec<Vec<BingoNumber>> = bingo_board.into_iter()
                                                    .filter(|&r| *r != "")
                                                    .map(|r| r.split_whitespace().map(|n| n.parse::<i8>().unwrap()))
                                                    .map(|r| r.map(|n| BingoNumber { value: n, is_marked: false } ).collect())
                                                    .collect();
    println!("clean bingo board: {:#?}", clean_bingo_board[0][0].value);
  }
}