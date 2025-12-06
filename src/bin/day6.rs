use aoc2025::akal_reader::read_lines;

#[derive(Debug, Clone)]
enum Operation {
    Add,
    Mult,
    Invalid,
}

#[derive(Debug, Clone)]
struct Problem {
    start_indx: usize,
    size: usize,
    op: Operation,
}

impl From<char> for Operation {
    fn from(s: char) -> Self {
        match s {
            '*' => Operation::Mult,
            '+' => Operation::Add,
            _ => Operation::Invalid,
        }
    }
}

fn main() {
    // get lines
    let lines = read_lines("../aoc-inputs-2025/day6-1.txt").expect("Cannot Read File");

    // Make a 2D matrix of numbers (still as strings)
    let lines_vec_part_one: Vec<Vec<String>> = lines
        .map_while(Result::ok)
        .map(|line| {
            line.split_whitespace()
                // filter out the extra spaces
                .filter(|&str| !str.is_empty())
                .map(|str| str.to_owned())
                .collect::<Vec<String>>()
        })
        .collect();

    // part one
    let mut sum = 0;

    // get the last line (with all the operators)
    if let Some(operations) = lines_vec_part_one.last() {
        // all the other rows in the Vec contain the actual values
        let values: Vec<Vec<usize>> = lines_vec_part_one[..(lines_vec_part_one.len() - 1)]
            .iter()
            .map(|line| {
                line.iter()
                    // map to number, filter out those that don't parse
                    .filter_map(|x| {
                        x.parse::<usize>()
                            .ok()
                    })
                    .collect()
            })
            // for each operation, get its index and string value
            .collect();
        for (indx, operation) in operations
            .iter()
            .enumerate()
        {
            // map each type to an operation
            match operation.as_str() {
                "*" => {
                    // execute a product on all numbers with the same index as the operator
                    sum += values
                        .iter()
                        .filter_map(|row| row.get(indx))
                        .product::<usize>();
                }
                "+" => {
                    // execute a sum on all numbers with the same index as the operator
                    sum += values
                        .iter()
                        .filter_map(|row| row.get(indx))
                        .sum::<usize>();
                }
                // should be invalid, panic if somehow ends up here
                _ => panic!("Unresolved operator: {}", operation),
            }
        }
    }

    // print out the sum for part one
    println!("Part 1: {}", sum);

    // part two
    // re-read the lines, this time keeping it as a simple 1D vector of strings (keep the line intact)
    let mut sum2 = 0;
    let lines = read_lines("../aoc-inputs-2025/day6-1.txt").expect("Cannot Read File");
    let lines_vec_part_two: Vec<String> = lines
        .map_while(Result::ok)
        .map(|line| {
            // but make sure to reverse the characters (for RTL)
            line.chars()
                .rev()
                .collect()
        })
        .collect();
    //
    // get the column sizes (moving right to left); the symbol is always in the left most place (skip the space after it)

    let last_line = lines_vec_part_two
        .last()
        .unwrap();
    let mut problems: Vec<Problem> = Vec::new();
    // enumerate over individual characters in the last line
    let mut chars = last_line
        .chars()
        .enumerate()
        .peekable();

    // this will keep track of the run length of a single problem (the number of columns the numbers cover)
    let mut len = 0;

    // while still having characters in the last line
    while let Some((indx, char)) = chars.next() {
        // increment the length
        len += 1;

        // since we reversed the line, the operator will be the last symbol in the run of a problem
        if char == '*' || char == '+' {
            // map it to the custom struct
            problems.push(Problem {
                start_indx: indx - len + 1,
                size: len,
                op: Operation::from(char),
            });
            // reset the run length
            len = 0;
            chars.next(); // Skip the following space, since the space following an operatotr
                          // is the separator between problems
        }
    }

    // get the values (excludes the last line)
    let values = &lines_vec_part_two[..(lines_vec_part_two.len() - 1)];
    // get the number of rows
    let num_rows = values.len();

    // iterator over problems
    for problem in problems.iter() {
        // store the numbers associated with this problem
        let mut problem_numbers: Vec<usize> = Vec::new();

        // to illustrate the following indices, consider this to be a problem
        // ------------------------------------
        // |                                 |
        //  |                               {0 to num_rows} // height
        // |                                 |
        // |                                 |
        // -------- {start_indx + len} -------- // run len

        // these are the column numbers for the digits
        for i in problem.start_indx..(problem.start_indx + problem.size) {
            // store the number as its being built
            let mut number: usize = 0;

            // iterate over the height
            for j in 0..num_rows {
                // get the digit
                let digit_char: char = values[j]
                    .chars()
                    .nth(i)
                    .unwrap();

                // if it is empty, there is nothing to do; skip
                if digit_char == ' ' {
                    continue;
                }

                // otherwise read it as a number
                let digit = digit_char
                    .to_digit(10)
                    .unwrap() as usize;

                // and build the number (*10 to move place digits);
                number *= 10;
                number += digit;
            }

            // push the number to the number store
            problem_numbers.push(number);
        }

        // now, execute an operation based on the operation type
        match problem.op {
            Operation::Add => {
                sum2 += problem_numbers
                    .iter()
                    .sum::<usize>();
            }
            Operation::Mult => {
                sum2 += problem_numbers
                    .iter()
                    .product::<usize>();
            }
            Operation::Invalid => {
                panic!("Invalid operation at {}", problem.start_indx);
            }
        }
    }

    println!("Part two: {}", sum2);
}
