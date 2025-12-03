use aoc2025::akal_reader::read_lines;
use std::collections::HashSet;

pub fn part1() -> u64 {
    let mut invalidIDs: Vec<u64> = vec![];
    if let Ok(lines) = read_lines("../aoc-inputs-2025/day2-1.txt") {
        let lines_vec: Vec<String> = lines.map_while(Result::ok).collect();

        // Split each line on a comma (get ranges)
        let ranges: Vec<&str> = lines_vec
            .iter()
            .flat_map(|line| line.split(','))
            .map(|s| s.trim())
            .collect();

        for range in ranges.iter() {
            let bounds: Vec<&str> = range.split("-").collect();
            let left_bound = bounds[0].parse::<u64>().unwrap();
            let right_bound = bounds[1].parse::<u64>().unwrap();

            for num in left_bound..=right_bound {
                // convert to string
                let num_as_str = num.to_string();

                // if it is not even, it does not count
                if num_as_str.len() % 2 != 0 {
                    continue;
                }

                let middle = num_as_str.len() / 2;
                let bigger_bite = &num_as_str[0..middle];
                let smaller_bite = &num_as_str[middle..];

                if bigger_bite == smaller_bite {
                    invalidIDs.push(num);
                }
            }
        }
    }
    println!("{:?}", invalidIDs);
    invalidIDs.iter().sum()
}

pub fn part2() -> u64 {
    let mut invalid_set: HashSet<u64> = HashSet::new();
    if let Ok(lines) = read_lines("../aoc-inputs-2025/day2-1.txt") {
        let lines_vec: Vec<String> = lines.map_while(Result::ok).collect();

        // Split each line on a comma (get ranges)
        let ranges: Vec<&str> = lines_vec
            .iter()
            .flat_map(|line| line.split(','))
            .map(|s| s.trim())
            .collect();

        for range in ranges.iter() {
            let bounds: Vec<&str> = range.split("-").collect();
            let left_bound = bounds[0].parse::<u64>().unwrap();
            let right_bound = bounds[1].parse::<u64>().unwrap();

            for num in left_bound..=right_bound {
                // convert to string
                let num_as_str = num.to_string();

                // the smallest a sequence be is of 1 digit and the largest it can
                // be is up to half the string.

                for len in 1..=num_as_str.len() / 2 {
                    let sequence = &num_as_str[..len];
                    // repeat the pattern so that it fits in the current length of the number
                    let invalid_number = sequence.repeat(num_as_str.len() / len);

                    if invalid_number == num_as_str {
                        invalid_set.insert(num);
                    }
                }
            }
        }
    }
    println!("{:?}", invalid_set);
    invalid_set.iter().sum()
}

pub fn main() {
    //println!("{}", part1());
    println!("{}", part2());
}
