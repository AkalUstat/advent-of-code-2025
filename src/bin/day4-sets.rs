use std::collections::HashSet;

use aoc2025::akal_reader::read_lines;

fn main() {
    // get lines
    if let Ok(lines) = read_lines("../aoc-inputs-2025/day4-1.txt") {
        let lines_vec: Vec<String> = lines.map_while(Result::ok).collect();

        // part 1
        println!("Part 1: {}", pt1(&lines_vec));

        // part 2
        println!("Part 2: {}", pt2(&lines_vec));
    }
}
fn pt1(lines: &[String]) -> usize {
    let set = set_of_all_rolls(lines);
    get_removable_set(&set).len()
}
fn pt2(lines: &[String]) -> usize {
    let mut sum = 0;

    let mut set = set_of_all_rolls(lines);

    // get the initial set of removable rolls
    let mut rem = get_removable_set(&set);

    // while the set of removable rolls isn't empty
    while !rem.is_empty() {
        // temp storage for affected neighbors
        let mut cascade_neighbors: HashSet<(usize, usize)> = HashSet::new();

        // for each
        for &rem in rem.iter() {
            // if this element has already been removed from the total set of rolls (meaning it
            // does not have a roll anymore), skip
            if !set.contains(&rem) {
                continue;
            }

            // get the neighbors in the larger set, of this position
            let neighbors = get_neighbors_set(&rem.0, &rem.1, &set);

            // if the current position passes the criteria
            if neighbors.len() <= 3 {
                // remove it from the set (roll removed)
                set.remove(&rem);

                // increment sum
                sum += 1;

                // add the neighbors (if valid) to the temp storage
                for neighbor in neighbors {
                    if set.contains(&neighbor) {
                        cascade_neighbors.insert(neighbor);
                    }
                }
            }
        }

        // replace set of removable rolls with the set of neighbors affected
        rem = cascade_neighbors;
    }

    sum
}

// get a set of all positions with rolls
fn set_of_all_rolls(lines: &[String]) -> HashSet<(usize, usize)> {
    lines
        .iter()
        .enumerate()
        .flat_map(|(line_indx, line)| {
            line.chars()
                .enumerate()
                // get the ones only with '@'
                .filter(|(_, ch)| *ch == '@')
                // store its (row#, col#)
                .map(move |(ch_indx, _)| (line_indx, ch_indx))
        })
        .collect()
}

// get the set of rolls which meet the criteria (<4 neighbors)
fn get_removable_set(roll_set: &HashSet<(usize, usize)>) -> HashSet<(usize, usize)> {
    roll_set
        .iter()
        .filter(|(row_indx, col_indx)| {
            // get its set of neighbors
            let neighbors_set = get_neighbors_set(row_indx, col_indx, roll_set);

            if neighbors_set.len() <= 3 {
                return true;
            }
            false
        })
        .cloned()
        .collect()
}

fn get_neighbors_set(
    row_indx: &usize,
    col_indx: &usize,
    complete_roll_set: &HashSet<(usize, usize)>,
) -> HashSet<(usize, usize)> {
    // get all possible neighbors via math
    let neighbors_set: HashSet<(usize, usize)> = (-1..=1)
        .flat_map(|dr| {
            (-1..=1).map(move |dc| {
                (
                    row_indx.checked_add_signed(dr),
                    col_indx.checked_add_signed(dc),
                )
            })
        })
        // get only valid neighbors (mathematically)
        .filter_map(|(dr, dc)| {
            match (dr, dc) {
                (Some(dr), Some(dc)) => Some((dr, dc)),
                _ => None, // Discard any tuple where either (or both) are None
            }
        })
        .filter(|&(dr, dc)| dr != *row_indx || dc != *col_indx) // Skip center cell
        .collect();

    // return this
    complete_roll_set
        .iter()
        // return onnly neighbors contained in the larger set of positions with rolls
        .filter(|(row_indx, col_indx)| neighbors_set.contains(&(*row_indx, *col_indx)))
        .cloned()
        .collect()
}
