use aoc2025::akal_reader::read_lines;

pub fn get_removeable_rolls(matrix: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let mut vector_of_removable_rolls: Vec<(usize, usize)> = vec![];

    let col_len = matrix.len();

    for (row_indx, row) in matrix.iter().enumerate() {
        let row_len = row.len();
        for (elem_indx, elem) in row.iter().enumerate() {
            if *elem != '@' {
                continue;
            }
            let mut num_roll_neighbors = 0;

            let row_indx_changes: [isize; 3] = [-1, 0, 1];
            let elem_indx_changes: [isize; 3] = [-1, 0, 1];

            for row_change in row_indx_changes.iter() {
                for elem_change in elem_indx_changes.iter() {
                    let new_pos = (
                        row_indx as isize + row_change,
                        elem_indx as isize + elem_change,
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
                vector_of_removable_rolls.push((row_indx, elem_indx));
            }
        }
    }

    vector_of_removable_rolls
}

pub fn pt1(lines: &Vec<String>) -> usize {
    let matrix: Vec<Vec<char>> = lines.iter().map(|line| line.chars().collect()).collect();

    return get_removeable_rolls(&matrix).len();
}

pub fn pt2(lines: &Vec<String>) -> usize {
    let mut removed_rolls = 0;
    let mut matrix: Vec<Vec<char>> = lines.iter().map(|line| line.chars().collect()).collect();

    let mut removeable_rolls = get_removeable_rolls(&matrix);

    while removeable_rolls.len() != 0 {
        for roll in removeable_rolls.iter() {
            matrix[roll.0][roll.1] = '.';
        }

        removed_rolls += removeable_rolls.len();
        removeable_rolls = get_removeable_rolls(&matrix);
    }

    removed_rolls
}

fn main() {
    // get lines
    if let Ok(lines) = read_lines("../aoc-inputs-2025/day4-1.txt") {
        let lines_vec: Vec<String> = lines.map_while(Result::ok).collect();

        // part 1
        println!("Part 1: {}", pt1(&lines_vec));
        println!("Part 2: {}", pt2(&lines_vec));
    }
}
