use aoc2025::akal_reader::read_lines;

pub fn find_largest_joltage(line: &String, num_digits: usize) -> u64 {
    let mut digits_remaining = num_digits;
    let mut last_digit_found_at_indx: i64 = -1;
    let mut digits: Vec<u8> = vec![];

    while digits_remaining != 0 {
        let substr = if last_digit_found_at_indx == -1 {
            &line[..line.len() - digits_remaining + 1]
        } else {
            &line[(last_digit_found_at_indx as usize + 1)..line.len() - digits_remaining + 1]
        };
        let next_digit = substr.bytes().map(|b| b - b'0').max().unwrap();
        let indx: u64 = substr.bytes().position(|b| b - b'0' == next_digit).unwrap() as u64;

        // let absolute_index = if last_digit_found_at_indx == -1 {
        //     indx
        // } else {
        //     indx + (last_digit_found_at_indx as u64) + 1
        // };

        digits.push(next_digit);
        // println!(
        //     "Digit at indx {}, {} of the new num is {}",
        //     absolute_index,
        //     total_digits - digits_remaining,
        //     next_digit
        // );
        digits_remaining -= 1;
        last_digit_found_at_indx = if last_digit_found_at_indx == -1 {
            indx as i64
        } else {
            last_digit_found_at_indx + indx as i64 + 1
        };
    }

    let mut proposed_number: u64 = 0;
    for digit in digits.iter() {
        proposed_number *= 10;
        proposed_number += *digit as u64;
    }

    // println!("Line: {}, max joltage {}", line, proposed_number);

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
