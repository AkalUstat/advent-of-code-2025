use aoc2025::akal_reader::read_lines;

fn main() {
    // get lines
    if let Ok(lines) = read_lines("../aoc-inputs-2025/day4-1sample.txt") {
        let lines_vec: Vec<String> = lines.map_while(Result::ok).collect();

        // part 1
        println!("Part 1: {}", pt1(&lines_vec));
        println!("Part 2: {}", pt2(&lines_vec));
    }
}

// use an array slice instead of vec; signals readonly + more adaptive
fn pt1(lines: &[String]) -> usize {
    let matrix: Vec<Vec<char>> = lines.iter().map(|line| line.chars().collect()).collect();

    get_removeable_rolls(&matrix).len()
}

fn pt2(lines: &[String]) -> usize {
    let mut removed_rolls = 0;
    let mut matrix: Vec<Vec<char>> = lines.iter().map(|line| line.chars().collect()).collect();

    // eagerly get new vector + check its not empty
    while let removeable_rolls = get_removeable_rolls(&matrix)
        && !removeable_rolls.is_empty()
    {
        // replace removed @ with .
        for &(row, col) in &removeable_rolls {
            matrix[row][col] = '.';
        }
        // add the length of the now-defunct vector to the sum of removed rolls
        removed_rolls += removeable_rolls.len();
    }

    removed_rolls
}

pub fn get_removeable_rolls(matrix: &[Vec<char>]) -> Vec<(usize, usize)> {
    let mut removable_rolls = Vec::new();
    let col_len = matrix.len();

    for (row_idx, row) in matrix.iter().enumerate() {
        let row_len = row.len();
        for (col_idx, &cell) in row.iter().enumerate() {
            if cell != '@' {
                continue;
            }

            let mut num_roll_neighbors = 0;

            for col_change in -1..=1 {
                for row_change in -1..=1 {
                    let new_row = row_idx as isize + col_change;
                    let new_col = col_idx as isize + row_change;

                    if !(new_row < 0 || new_row as usize > col_len - 1)
                        && !(new_col < 0 || new_col as usize > row_len - 1)
                    {
                        if matrix[new_row as usize][new_col as usize] == '@' {
                            num_roll_neighbors += 1;
                        }
                    }
                }
            }

            if num_roll_neighbors <= 4 {
                removable_rolls.push((row_idx, col_idx));
            }
        }
    }

    removable_rolls
}
