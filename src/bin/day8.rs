use aoc2025::akal_reader::read_lines;
use std::collections::{HashMap, HashSet};
use std::str::FromStr;

// x, y, z
#[derive(Debug, PartialEq)]
struct Position(usize, usize, usize);
impl FromStr for Position {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s
            .split(',')
            .collect();

        if parts.len() == 3 {
            let x_str = parts[0].trim();
            let y_str = parts[1].trim();
            let z_str = parts[2].trim();

            match (
                x_str.parse::<usize>(),
                y_str.parse::<usize>(),
                z_str.parse::<usize>(),
            ) {
                (Ok(x), Ok(y), Ok(z)) => Ok(Position(x, y, z)),
                _ => Err(format!("Invalid number format in input: {}", s)),
            }
        } else {
            Err(format!("Invalid point format. Expected 'x,y', got: {}", s))
        }
    }
}

fn euclidean_distance(pos1: &Position, pos2: &Position) -> usize {
    let dx = pos2
        .0
        .abs_diff(pos1.0);
    let dy = pos2
        .1
        .abs_diff(pos1.1);
    let dz = pos2
        .2
        .abs_diff(pos1.2);
    let sq_dist = dx.pow(2) + dy.pow(2) + dz.pow(2);

    usize::isqrt(sq_dist)
}

fn create_pairs(positions: &Vec<Position>) -> Vec<Position> {
    Vec::new()
}

fn part1() -> usize {
    let mut positions: Vec<Position> = read_lines("../aoc-inputs-2025/day8sample.txt")
        .expect("Cannot read file")
        .map_while(Result::ok)
        .flat_map(|line| Position::from_str(&line))
        .collect();

    0
}
fn part2() -> usize {
    0
}
fn main() {
    println!("{}", part1());
    println!("{}", part2());
}
