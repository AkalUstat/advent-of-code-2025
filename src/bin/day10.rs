use aoc2025::akal_reader::read_lines;
use regex::Regex;
use std::collections::{HashSet, VecDeque};
use std::str::FromStr;

use std::sync::OnceLock;
static LINE_REGEX: OnceLock<Regex> = OnceLock::new();

#[derive(Debug, Clone)]
struct Line {
    schematic: u32,
    buttons: Vec<u32>,
    joltages: Vec<usize>,
}

impl FromStr for Line {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = LINE_REGEX.get_or_init(|| {
            Regex::new(r"(\[(?:\.|#)+\])\s((?:\(\d+(?:,\d+)*\)\s)+)(\{\d+(?:,\d+)*\})").unwrap()
        });

        let caps = re
            .captures(s)
            .ok_or_else(|| format!("Line does not match expected format: {s}"))?;

        // get data
        let schematic_str = &caps[1];

        let schematic: u32 = schematic_str[1..schematic_str.len() - 1]
            .chars()
            .rev() // since buttons are built RTL, build the schematic RTL as well
            .fold(0u32, |acc, ch| {
                let ls = acc << 1;
                // left shift
                match ch {
                    '#' => ls + 1,
                    _ => ls,
                }
            });

        let buttons_str = &caps[2];
        let buttons: Vec<u32> = buttons_str
            .split(['(', ')'])
            .filter(|s| {
                !s.trim()
                    .is_empty()
            })
            .map(|button_str| {
                button_str
                    .split(',')
                    .filter_map(|digit| {
                        digit
                            .trim()
                            .parse::<u32>()
                            .ok()
                    })
                    .fold(0u32, |acc, bit_pos| acc | (1 << bit_pos))
            })
            .collect();

        let joltages_str = &caps[3];
        let joltages: Vec<usize> = joltages_str[1..joltages_str.len() - 1]
            .split(',')
            .filter_map(|num| {
                num.parse::<usize>()
                    .ok()
            })
            .collect();

        

        Ok(Line {
            schematic,
            buttons,
            joltages,
        })
    }
}

// find the fastest way to get to end
fn bfs(end_state: u32, buttons: &[u32]) -> usize {
    let mut current_level: VecDeque<u32> = VecDeque::from([0]);

    let mut visited_states: HashSet<u32> = HashSet::new();

    let mut steps = 0;
    while !current_level.is_empty() {
        let mut next_level: VecDeque<_> = VecDeque::new();

        for state in current_level {
            if state == end_state {
                return steps;
            }

            for button in buttons {
                let new_state = state ^ button;
                if !visited_states.contains(&new_state) {
                    next_level.push_back(new_state);
                    visited_states.insert(new_state);
                }
            }
        }
        current_level = next_level;
        steps += 1;
    }

    steps
}
fn part1() -> usize {
    let lines: Vec<Line> = read_lines("../aoc-inputs-2025/day10.txt")
        .expect("Cannot read file")
        .map_while(Result::ok)
        .filter_map(|line| Line::from_str(&line).ok())
        .collect();

    lines
            .iter()
            .map(|line| bfs(line.schematic, &line.buttons))
            .sum::<usize>()
}

fn part2() -> usize {

    // I gave up on Part 2; had to consult AI on 
    // how to do it and learn the process. 
    // so, I'm not including it here.
    0
}
fn main() {
    println!("{}", part1());
    println!("{}", part2());
}
