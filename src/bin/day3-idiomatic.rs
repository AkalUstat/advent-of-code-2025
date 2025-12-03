// Since I am learning Rust while doing AOC this year,
// I want to refactor my solutions after they work to learn the idiomatic
// ways of doing Rust.
//
use aoc2025::akal_reader::read_lines;
pub fn find_largest_joltage(line: &String, num_digits: usize) -> u64 {
    let mut digits_remaining = num_digits;
    // None indicates that there is no previous value here
    let mut last_digit_found_at_indx: Option<usize> = None;
    let mut digits: Vec<u8> = vec![];

    while digits_remaining != 0 {
        // if no previous value, set the start to 0; map can transform Option values.
        let start = last_digit_found_at_indx.map(|i| i + 1).unwrap_or(0);
        let end = line.len() - digits_remaining + 1;
        let substr = &line[start..end];

        let next_digit = substr.bytes().map(|b| b - b'0').max().unwrap();
        let indx = substr.bytes().position(|b| b - b'0' == next_digit).unwrap();
        let absolute_index = start + indx;

        digits.push(next_digit);

        digits_remaining -= 1;
        last_digit_found_at_indx = Some(absolute_index);
    }
    let mut proposed_number: u64 = 0;
    for digit in digits.iter() {
        proposed_number *= 10;
        proposed_number += *digit as u64;
    }
    proposed_number
}

pub fn iterate_lines(lines: &Vec<String>, num_digits: usize) -> u64 {
    lines
        .iter()
        .map(|line| find_largest_joltage(line, num_digits))
        .sum()
}

pub fn main() {
    // get lines
    if let Ok(lines) = read_lines("../aoc-inputs-2025/day3-1.txt") {
        let lines_vec: Vec<String> = lines.map_while(Result::ok).collect();

        // part 1
        println!("Part 1: {}", iterate_lines(&lines_vec, 2));

        // part 2
        println!("Part 2: {}", iterate_lines(&lines_vec, 12));
    }
}
