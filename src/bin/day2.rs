use aoc2025::akal_reader::read_lines;
use itertools::Itertools;

fn create_digit_range(left_min: char, right_max: char, is_last: bool) -> Vec<char> {
    let mut valid_digits = vec![];

    if is_last && (left_min == '0') && (right_max == '0') {
        for j in '0'..='9' {
            valid_digits.push(j);
        }
    } else if right_max < left_min {
        for j in right_max..=left_min {
            valid_digits.push(j);
        }
    } else {
        for j in left_min..=right_max {
            valid_digits.push(j);
        }
    }

    valid_digits
}

fn generate_combination_strings(data: Vec<Vec<char>>) -> Vec<String> {
    // 1. Convert the outer Vec into an iterator of inner Vecs
    let iter_of_vecs = data.into_iter();

    // 2. Map each inner Vec into its own IntoIter<char>
    let iter_of_iters = iter_of_vecs.map(|v| v.into_iter());

    // 3. Collect the map iterator results into a Vec first
    let list_of_iters: Vec<_> = iter_of_iters.collect();

    // 4. Turn the Vec back into an iterator, then call the method.
    // The .multi_cartesian_product() method expects an iterator of iterators.
    let product_iter = list_of_iters.into_iter().multi_cartesian_product();

    // 5. Collect each resulting Vec<char> into a String
    let result_strings: Vec<String> = product_iter
        .map(|combination_vec: Vec<char>| combination_vec.into_iter().collect::<String>())
        .collect();

    result_strings
}

pub fn part1() -> i64 {
    let mut invalidIDs: Vec<i64> = vec![];
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
            let left_bound_digits: Vec<char> = bounds[0].chars().collect();
            let right_bound_digits: Vec<char> = bounds[1].chars().collect();

            let left_bound_as_number = bounds[0].parse::<i64>().unwrap();
            let right_bound_as_number = bounds[1].parse::<i64>().unwrap();

            //  println!("{:?}", range);

            let left_bound_len = left_bound_digits.len();
            let right_bound_len = right_bound_digits.len();

            if left_bound_len % 2 == 1 && right_bound_len % 2 == 1 {
                // if both are uneven, then move to next iteration; we will
                // not have invalid IDs
                continue;
            }

            let mut digit_bounds: Vec<Vec<char>> = vec![];

            // if uneven bounds (right will always be greater than left)
            if left_bound_len != right_bound_len {
                // one of them will be uneven:
                if left_bound_len % 2 == 1 {
                    for i in 0..right_bound_len {
                        if i == 0 {
                            let min = right_bound_digits[i];
                            let max = right_bound_digits[i];
                            digit_bounds.push(create_digit_range(min, max, false));
                        } else {
                            let min = '0';
                            let max = right_bound_digits[i];
                            digit_bounds.push(create_digit_range(
                                min,
                                max,
                                i == (right_bound_len - 1),
                            ));
                        }
                    }
                } else if right_bound_len % 2 == 1 {
                    for i in 0..left_bound_len {
                        let min = left_bound_digits[i];
                        let max = '9';
                        digit_bounds.push(create_digit_range(min, max, i == (right_bound_len - 1)));
                    }
                }
                // println!("Range {:?} digits: {:?}", range, digit_bounds);
            } else {
                // even bounds
                for i in 0..left_bound_len {
                    let left_min = left_bound_digits[i];
                    let right_max = right_bound_digits[i];

                    digit_bounds.push(create_digit_range(
                        left_min,
                        right_max,
                        i == (right_bound_len - 1),
                    ));
                }

                // println!("Range {:?} digits: {:?}", range, digit_bounds);
            }

            let half_len = digit_bounds.len() / 2;

            let slice: Vec<Vec<char>> = digit_bounds[0..half_len].to_vec();
            let permutations = generate_combination_strings(slice);

            for perm in permutations.iter() {
                let mut doubled = String::new();

                doubled.push_str(perm);
                doubled.push_str(perm);

                let as_number = doubled.parse::<i64>().unwrap();

                if as_number >= left_bound_as_number && as_number <= right_bound_as_number {
                    invalidIDs.push(as_number);
                }
            }
        }
    }
    println!("{:?}", invalidIDs);
    invalidIDs.iter().sum()
}

pub fn main() {
    println!("{}", part1());
}
