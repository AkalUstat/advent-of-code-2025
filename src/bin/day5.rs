use std::collections::HashSet;
use std::ops::RangeInclusive;

use aoc2025::akal_reader::read_lines;
use itertools::Itertools;

#[derive(Debug, Clone)]
struct AkalRange {
    start: usize,
    end: usize,
}

impl AkalRange {
    fn update_bounds(self, new_range: &Self) -> Self {
        Self {
            start: self.start.min(new_range.start),
            end: self.end.max(new_range.end),
        }
    }

    fn create_range(&self) -> RangeInclusive<usize> {
        (self.start)..=(self.end)
    }

    fn does_overlap(&self, other: &Self) -> bool {
        !(self.end < other.start || other.end < self.start)
    }
}

fn main() {
    // get lines
    let lines = read_lines("../aoc-inputs-2025/day5-1.txt").expect("Cannot Read File");
    let lines_vec: Vec<String> = lines.map_while(Result::ok).collect();

    if let Some(separator_indx) = lines_vec.iter().position(|elem| elem == "") {
        let (fresh_ranges, ids) = lines_vec.split_at(separator_indx);
        // get rid of the empty line that is the separator
        let available_ids: Vec<usize> = ids
            .iter()
            .filter(|el| !el.is_empty())
            .map(|str| str.parse::<usize>())
            .flatten()
            .collect();

        let mut bounds: Vec<AkalRange> = fresh_ranges
            .iter()
            .map(|range| {
                range
                    .split("-")
                    .map(|s| s.parse::<usize>())
                    .flatten() // removes nones
                    .collect::<Vec<usize>>()
            })
            .map(|bounds_vec| AkalRange {
                start: bounds_vec[0],
                end: bounds_vec[1],
            })
            .collect::<Vec<AkalRange>>();

        // sort the bounds by the start value, so that we only need to compare to the previous
        // bounds for overlap
        bounds.sort_by_key(|range| range.start);

        let ranges: Vec<RangeInclusive<usize>> = bounds
            .into_iter()
            .fold(Vec::new(), |mut acc, x| {
                if acc.is_empty() {
                    acc.push(x);
                } else if let Some(last_val) = acc.last().cloned() {
                    if x.does_overlap(&last_val) {
                        acc.pop();
                        let new = x.update_bounds(&last_val);
                        acc.push(new);
                    } else {
                        acc.push(x);
                    }
                }

                return acc;
            })
            .iter()
            .map(|akal_range| akal_range.create_range())
            .collect();

        // part 1
        let sum = available_ids.iter().fold(0, |acc, x| {
            if let Some(_) = ranges.iter().find(|range| range.contains(x)) {
                return acc + 1;
            }

            acc
        });
        println!("{}", sum);
    }
}
