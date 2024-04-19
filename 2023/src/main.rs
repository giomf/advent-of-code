use std::fs;

mod day01;
const INPUT_BASE_PATH: &str = "inputs/input";
trait Puzzle {
    fn solve_first_part(&self) -> i32;
    fn solve_second_part(&self) -> i32;
}

pub fn get_input(day: u8) -> String {
    let input_path = format!("{INPUT_BASE_PATH}_{day}");
    fs::read_to_string(&input_path).expect(&format!("Unable to read {input_path}"))
}

fn main() {
    let puzzles: Vec<Box<dyn Puzzle>> = vec![Box::new(day01::Day::new(get_input(1)))];

    for (i, puzzle) in puzzles.iter().enumerate() {
        println!("Day {:02} Part 1: {}", i + 1, puzzle.solve_first_part());
        println!("Day {:02} Part 2: {}", i + 1, puzzle.solve_second_part());
        println!("---");
    }
}
