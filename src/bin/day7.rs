use aoc2025::akal_reader::read_lines;
use std::collections::{HashMap, HashSet};

enum Cases {
    Split,
    End,
}

// a beam travels until it reaches the end or is split
fn pt1_travel(matrix: &Vec<Vec<char>>, beam: &(usize, usize)) -> (usize, usize, Cases) {
    let mut position = beam.clone();

    // while not the end
    while position.0 != matrix.len() - 1 {
        // check the next symbol
        let next_symbol = matrix[position.0 + 1][position.1];

        // if it is a splitter, return the split condition + its position
        if next_symbol == '^' {
            return (position.0 + 1, position.1, Cases::Split);
        }

        // otherwise, keep moving down the matrix
        position.0 += 1;
    }

    // in the end case
    (position.0, position.1, Cases::End)
}

fn part1() -> usize {
    let lines = read_lines("../aoc-inputs-2025/day7sample.txt").expect("Cannot Read File");
    // make a 2D character matrix from the input
    let lines_vec: Vec<Vec<char>> = lines
        .map_while(Result::ok)
        .map(|line| {
            line.chars()
                .collect()
        })
        .collect();
    // y, x
    // this is a moving tracker of beams that are active
    let mut traveling_beams: HashSet<(usize, usize)> = HashSet::new();
    // keeps a set of visited splitters (at the end, we can just count this)
    let mut visited_splitters: HashSet<(usize, usize)> = HashSet::new();

    // find the S in the first line
    let s_index = lines_vec[0]
        .iter()
        .position(|&c| c == 'S')
        .unwrap_or(0);

    // insert the start position into traveling beams
    traveling_beams.insert((0, s_index));

    // while there are still active beams
    while traveling_beams.len() != 0 {
        // keep track of the next set
        let mut next_beams: HashSet<(usize, usize)> = HashSet::new();

        // for each beam
        for beam in &traveling_beams {
            // travel
            let (pos0, pos1, case) = pt1_travel(&lines_vec, beam);

            match case {
                // if it comes across a split
                Cases::Split => {
                    // store the position of the splitter
                    visited_splitters.insert((pos0, pos1));

                    // put the two new beams into the new set; not including
                    // the old beam will remove it from the set, in practice.
                    next_beams.insert((pos0, pos1 - 1));
                    next_beams.insert((pos0, pos1 + 1));
                }
                // in the end case, there is nothing to do
                Cases::End => {}
            }
        }

        // update the traveling beams set with the new set.
        traveling_beams = next_beams;
    }

    // return the number of splitters visited.
    visited_splitters.len()
}

fn part2() -> usize {
    let lines = read_lines("../aoc-inputs-2025/day7.txt").expect("Cannot Read File");
    let lines_vec: Vec<Vec<char>> = lines
        .map_while(Result::ok)
        .map(|line| {
            line.chars()
                .collect()
        })
        .collect();
    // y, x, timelines
    let mut traveling_beams: HashSet<(usize, usize)> = HashSet::new();
    let mut visited_splitters: HashSet<(usize, usize)> = HashSet::new();

    // use a map to store counts (timelines) for each position
    // at the end, this should only have beams at the last row, with the timeline counts
    // since it cascades downwards.
    let mut beams: HashMap<(usize, usize), usize> = HashMap::new();

    // find the S in the first line
    let s_index = lines_vec[0]
        .iter()
        .position(|&c| c == 'S')
        .unwrap_or(0);

    traveling_beams.insert((0, s_index));

    // add the first beam with 1 timeline to the map
    beams.insert((0, s_index), 1);

    // while there are still beams
    while traveling_beams.len() != 0 {
        let mut next_beams: HashSet<(usize, usize)> = HashSet::new();

        for beam in &traveling_beams {
            let (pos0, pos1, case) = pt1_travel(&lines_vec, beam);

            match case {
                Cases::Split => {
                    visited_splitters.insert((pos0, pos1));

                    next_beams.insert((pos0, pos1 - 1));

                    // get the number of timelines as it was at the parent beam
                    let parent_val = beams
                        .get(beam)
                        .unwrap()
                        .clone();

                    // for the left beam, give it the same count (if it
                    // is already there - aka visited by another beam already -
                    // add to the count that exists)
                    beams
                        .entry((pos0, pos1 - 1))
                        .and_modify(|count| *count += parent_val)
                        .or_insert(parent_val);

                    next_beams.insert((pos0, pos1 + 1));

                    // same for the right beam
                    beams
                        .entry((pos0, pos1 + 1))
                        .and_modify(|count| *count += parent_val)
                        .or_insert(parent_val);

                    // remove the parent beam, so that timelines are not double-
                    // conuted.
                    beams.remove(beam);
                }

                Cases::End => {}
            }
        }

        traveling_beams = next_beams;
    }

    // return the sum of timelines.
    beams
        .values()
        .sum::<usize>()
}
fn main() {
    println!("{}", part1());
    println!("{}", part2());
}
