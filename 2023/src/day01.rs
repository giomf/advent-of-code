use crate::Puzzle;
use core::panic;
use regex::Regex;
use std::ops::Range;

pub struct Day {
    input: String,
}

impl Day {
    pub fn new(input: String) -> Self {
        Self { input }
    }
}

impl Puzzle for Day {
    fn solve_first_part(&self) -> i32 {
        let lines: Vec<&str> = self.input.split('\n').collect();
        lines
            .iter()
            .filter(|line| !line.is_empty())
            .map(|line| get_first_last_digit_as_number(line))
            .reduce(|acc, x| acc + x)
            .expect("Failed to count numbers")
    }

    fn solve_second_part(&self) -> i32 {
        let lines: Vec<&str> = self.input.split('\n').collect();
        lines
            .iter()
            .filter(|line| !line.is_empty())
            .map(|line| {
                let replaced_line = replace_word_by_digits(line);
                // print!("{} -> {}", line, &replaced_line);
                get_first_last_digit_as_number(&replaced_line)
            })
            .reduce(|acc, x| {
                // println!(" :{x} = {acc}");
                acc + x
            })
            .expect("Failed to count numbers")
    }
}

fn get_first_last_digit_as_number(line: &str) -> i32 {
    let first = line
        .chars()
        .find(|x| x.is_digit(10))
        .expect(&format!("No first digit found in {line}"));

    let last = line
        .chars()
        .rfind(|x| x.is_digit(10))
        .expect(&format!("No last digit found in {line}"));

    (first.to_string() + &last.to_string())
        .parse::<i32>()
        .unwrap()
}

fn replace_word_by_digits(line: &str) -> String {
    let re_first = Regex::new(r"(one|two|three|four|five|six|seven|eight|nine).*$")
        .expect("Failed to construct regex");

    let re_last = Regex::new(r"^(.*)(one|two|three|four|five|six|seven|eight|nine)")
        .expect("Failed to construct regex");

    let mut buffer = line.to_string();
    let first = re_first.captures(line);
    let last = re_last.captures(line);

    if let Some(first) = first {
        let first = first.get(first.len() - 1).unwrap();
        buffer = replace_word_by_digits_in_line(buffer, first.as_str(), first.range());
        if let Some(last) = last {
            let last = last.get(last.len() - 1).unwrap();
            if first != last {
                let range = calculate_range(first.range(), last.range());
                buffer = replace_word_by_digits_in_line(buffer, last.as_str(), range);
            }
        };
    };

    buffer
}

fn calculate_range(first: Range<usize>, last: Range<usize>) -> Range<usize> {
    if first.end < last.start {
        (last.start - first.len() + 1)..(last.end - first.len() + 1)
    } else {
        (first.start + 1)..(first.start + last.len())
    }
}

fn replace_word_by_digits_in_line(mut line: String, word: &str, range: Range<usize>) -> String {
    let digit = match_word_to_digit(word).to_string();
    line.replace_range(range, &digit);
    line
}

fn match_word_to_digit(word: &str) -> i32 {
    match word {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => panic!("No matching word to digit"),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_first_part() {
        let input = "\
            1abc2\n\
            pqr3stu8vwx\n\
            a1b2c3d4e5f\n\
            treb7uchet";
        let puzzle = Day::new(input.to_string());
        assert_eq!(142, puzzle.solve_first_part())
    }
    #[test]
    fn test_second_part() {
        let input = "\
            two1nine\n\
            eightwothree\n\
            abcone2threexyz\n\
            xtwone3four\n\
            4nineeightseven2\n\
            zoneight234\n\
            7pqrstsixteen";
        let puzzle = Day::new(input.to_string());
        // assert_eq!(281, puzzle.solve_second_part())
        assert_eq!(281, puzzle.solve_second_part())
    }
}
