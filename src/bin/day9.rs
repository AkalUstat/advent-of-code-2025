use std::str::FromStr;

use aoc2025::akal_reader::read_lines;

#[derive(PartialEq, Debug, Copy, Clone)]
struct Point {
    x: usize,
    y: usize
}

impl FromStr for Point {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x_str, y_str) = s
            .split_once(',')
            .ok_or_else(|| format!("Invalid point format. Expected 'x,y', got: {s}"))?;
        
        let x = x_str.trim().parse()
            .map_err(|_| format!("Invalid x coordinate: {x_str}"))?;
        let y = y_str.trim().parse()
            .map_err(|_| format!("Invalid y coordinate: {y_str}"))?;
        
        Ok(Point { x, y })
    }
}

fn part1() -> usize {
    let red_tiles: Vec<Point> = read_lines("../aoc-inputs-2025/day9.txt")
        .expect("Cannot read file")
        .map_while(Result::ok)
        .filter_map(|line| line.parse().ok())
        .collect();


    red_tiles
        .iter()
        .enumerate()
        .flat_map(|(i, &tile)| {
            red_tiles[i + 1..]
                .iter()
                .map(move |&other_tile| {
                    (tile.x.abs_diff(other_tile.x) + 1) 
                        * (tile.y.abs_diff(other_tile.y) + 1)
                })
        })
        .max()
        .unwrap_or(0)


}
fn part2() -> usize {0}

fn main() {
    println!("{}", part1());
    println!("{}", part2());
}
