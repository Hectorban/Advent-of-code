use std::fs;
use ansi_term::Colour::{Red, Blue};

#[derive(Debug)]
struct BingoNumber {
  value: i8,
  is_marked: bool
}

fn main(){
  let input = fs::read_to_string("inputs/7-8.txt").expect("Problem while reading from file."); 
  let mut input_lines: Vec<&str> = input.lines().collect();
  let bingo_inputs: Vec<i8> = input_lines.remove(0).split(",").map(|n| n.parse::<i8>().unwrap()).collect();
  let mut prepared_bingo_boards = prepare_bingo_boards(input_lines);
  let first_bingo_board = &mut prepared_bingo_boards[0];
  display_board_with_marks(first_bingo_board);
  for (index, bingo_number) in bingo_inputs.into_iter().enumerate() {
    first_bingo_board.into_iter().for_each(|row| {
      row.iter_mut().for_each(|n| if n.value == bingo_number {n.is_marked = true} )
    });
    display_board_with_marks(first_bingo_board)
  }
}

fn display_board_with_marks(bingo_board: &mut Vec<Vec<BingoNumber>>){
  for row in bingo_board {
    println!("");
    for number in row {
      let formatted_number = if number.is_marked {Red.paint(format!("{}", number.value))} else {Blue.paint(format!("{}", number.value))};
      print!(" {}", formatted_number)
    } 
  }
  println!("")
}

fn prepare_bingo_boards(input_lines: Vec<&str>) -> Vec<Vec<Vec<BingoNumber>>>{
  let mut clean_bingo_boards = vec![];
  for bingo_board in input_lines.chunks(6) {
    let clean_bingo_board: Vec<Vec<BingoNumber>> = bingo_board.into_iter()
                                                    .filter(|&r| *r != "")
                                                    .map(|r| r.split_whitespace().map(|n| n.parse::<i8>().unwrap()))
                                                    .map(|r| r.map(|n| BingoNumber { value: n, is_marked: false } ).collect())
                                                    .collect();
    clean_bingo_boards.push(clean_bingo_board)
  }
  clean_bingo_boards
} 