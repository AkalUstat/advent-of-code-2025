use aoc2025::akal_reader::read_lines;
use std::collections::{HashMap, HashSet};

type Position = (usize, usize);

enum TravelResult {
    Split(Position),
    End,
}

fn travel_beam(matrix: &[Vec<char>], start: Position) -> TravelResult {
    let (mut row, col) = start;
    let max_row = matrix.len() - 1;

    while row < max_row {
        row += 1;
        if matrix[row][col] == '^' {
            return TravelResult::Split((row, col));
        }
    }

    TravelResult::End
}

fn find_start_column(grid: &[Vec<char>]) -> Option<usize> {
    grid.first()?
        .iter()
        .position(|&c| c == 'S')
}

fn read_grid(path: &str) -> Vec<Vec<char>> {
    read_lines(path)
        .expect("Cannot read file")
        .map_while(Result::ok)
        .map(|line| {
            line.chars()
                .collect()
        })
        .collect()
}

fn part1() -> usize {
    let grid = read_grid("../aoc-inputs-2025/day7sample.txt");
    let start_col = find_start_column(&grid).expect("No start position found");

    let mut active_beams = HashSet::from([(0, start_col)]);
    let mut visited_splitters = HashSet::new();

    while !active_beams.is_empty() {
        active_beams = active_beams
            .iter()
            .filter_map(|&beam| match travel_beam(&grid, beam) {
                TravelResult::Split(pos) => {
                    visited_splitters.insert(pos);
                    Some(vec![(pos.0, pos.1 - 1), (pos.0, pos.1 + 1)])
                }
                TravelResult::End => None,
            })
            .flatten()
            .collect();
    }

    visited_splitters.len()
}

fn part2() -> usize {
    let grid = read_grid("../aoc-inputs-2025/day7.txt");
    let start_col = find_start_column(&grid).expect("No start position found");

    let mut active_beams = HashSet::from([(0, start_col)]);
    let mut beam_counts: HashMap<Position, usize> = HashMap::from([((0, start_col), 1)]);

    while !active_beams.is_empty() {
        let mut next_beams = HashSet::new();

        for &beam in &active_beams {
            if let TravelResult::Split(pos) = travel_beam(&grid, beam) {
                let parent_count = beam_counts
                    .remove(&beam)
                    .unwrap_or(0);

                for &new_beam in &[(pos.0, pos.1 - 1), (pos.0, pos.1 + 1)] {
                    next_beams.insert(new_beam);
                    *beam_counts
                        .entry(new_beam)
                        .or_insert(0) += parent_count;
                }
            }
        }

        active_beams = next_beams;
    }

    beam_counts
        .values()
        .sum()
}

fn main() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}
