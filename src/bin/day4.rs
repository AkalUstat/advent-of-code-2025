use aoc2025::akal_reader::read_lines;

pub fn traverse_graph(lines: &Vec<String>) -> usize {
    let mut sum = 0;
    let matrix: Vec<Vec<char>> = lines.iter().map(|line| line.chars().collect()).collect();

    let col_len = matrix.len();

    for (row_indx, row) in matrix.iter().enumerate() {
        let row_len = row.len();
        for (elem_indx, elem) in row.iter().enumerate() {
            if *elem != '@' {
                continue;
            }
            let mut num_roll_neighbors = 0;

            let col_indx_changes: [isize; 3] = [-1, 0, 1];
            let row_indx_changes: [isize; 3] = [-1, 0, 1];

            for col_change in col_indx_changes.iter() {
                for row_change in row_indx_changes.iter() {
                    let new_pos = (
                        row_indx as isize + col_change,
                        elem_indx as isize + row_change,
                    );

                    if !(new_pos.0 < 0 || new_pos.0 as usize > col_len - 1)
                        && !(new_pos.1 < 0 || new_pos.1 as usize > row_len - 1)
                    {
                        if matrix[new_pos.0 as usize][new_pos.1 as usize] == '@' {
                            num_roll_neighbors += 1;
                        }
                    }
                }
            }

            if num_roll_neighbors <= 4 {
                sum += 1;
            }
        }
    }
    sum
}

fn main() {
    // get lines
    if let Ok(lines) = read_lines("../aoc-inputs-2025/day4-1.txt") {
        let lines_vec: Vec<String> = lines.map_while(Result::ok).collect();

        // part 1
        println!("Part 1: {}", traverse_graph(&lines_vec));
    }
}
