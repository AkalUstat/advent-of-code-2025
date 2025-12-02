use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn part1() -> i32 {
    let file = File::open("../aoc-inputs-2025/day1-1.txt").unwrap();
    let reader = BufReader::new(file);

    let mut start_point = 50;
    let mut total_times_at_zero = 0;

    let mut lines_iter = reader.lines().map(|l| l.unwrap());

    for line in lines_iter {
        let line_val = &line;
        let dir = line_val.chars().nth(0);
        let num_places = &line_val[1..];
        let parsed_num = num_places.parse::<i32>().unwrap();

        if let Some('L') = dir {
            println!("Left {} places", num_places);
            start_point = (start_point - parsed_num).rem_euclid(100);

            if start_point == 0 {
                total_times_at_zero += 1;
            }
            println!("Ending point {}", start_point);
        } else if let Some('R') = dir {
            println!("Right {} places", num_places);
            start_point = (start_point + parsed_num).rem_euclid(100);

            if start_point == 0 {
                total_times_at_zero += 1;
            }
            println!("Ending point {}", start_point);
        } else {
            panic!("Direction Read Failed");
        }
    }

    total_times_at_zero
}

pub fn part2() -> i32 {
    let file = File::open("../aoc-inputs-2025/day1-1.txt").unwrap();
    let reader = BufReader::new(file);

    let mut start_point = 50;
    let mut total_times_at_zero = 0;

    let mut lines_iter = reader.lines().map(|l| l.unwrap());

    for line in lines_iter {
        let line_val = &line;
        let (dir, places) = line_val.split_at(1);
        let sign = if dir == "L" { -1 } else { 1 };
        let dist = places.parse::<i32>().unwrap();

        let end_pos = (start_point + sign * dist).rem_euclid(100).abs();
        let mut clicks = 0;
        if dir == "L" {
            // left moves counter clockwise, so the calculation isn't as easy.
            // We first get the distance from 0 (to make it the same calculation as clockwise)
            let reversed = (100 - start_point) % 100;
            // then get the number of times we pass 0
            clicks += (reversed + dist) / 100;
        } else {
            clicks += (start_point + dist) / 100;
        }

        total_times_at_zero += clicks;

        start_point = end_pos;
    }

    total_times_at_zero
}

fn main() {
    // println!("{:?} times", part1());
    println!("{:?} times", part2());
}
