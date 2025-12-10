use aoc2025::akal_reader::read_lines;
use std::collections::{HashMap, HashSet, VecDeque};
use std::str::FromStr;

#[derive(Clone, Copy, PartialEq)]
enum CellType {
    Empty,
    Red,
    Green,
    Outside,
}

#[derive(Hash, Eq, PartialEq)]
struct LineSegment {
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize,
}

#[derive(PartialEq, Debug, Copy, Clone, Hash)]
struct Point {
    x: usize,
    y: usize,
}

impl FromStr for Point {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x_str, y_str) = s
            .split_once(',')
            .ok_or_else(|| format!("Invalid point format. Expected 'x,y', got: {s}"))?;

        let x = x_str
            .trim()
            .parse()
            .map_err(|_| format!("Invalid x coordinate: {x_str}"))?;
        let y = y_str
            .trim()
            .parse()
            .map_err(|_| format!("Invalid y coordinate: {y_str}"))?;

        Ok(Point { x, y })
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.y
            .cmp(&other.y)
            .then(
                self.x
                    .cmp(&other.x),
            )
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Point {}

fn part1() -> usize {
    let red_tiles: Vec<Point> = read_lines("../aoc-inputs-2025/day9.txt")
        .expect("Cannot read file")
        .map_while(Result::ok)
        .filter_map(|line| {
            line.parse()
                .ok()
        })
        .collect();

    red_tiles
        .iter()
        .enumerate()
        .flat_map(|(i, &tile)| {
            red_tiles[i + 1..]
                .iter()
                .map(move |&other_tile| {
                    (tile
                        .x
                        .abs_diff(other_tile.x)
                        + 1)
                        * (tile
                            .y
                            .abs_diff(other_tile.y)
                            + 1)
                })
        })
        .max()
        .unwrap_or(0)
}

/*
This was a new algorithm for me, so I consulted claude code.
I did not have it give me the algorithm. Instead, I asked it to guide me
to it using questions. That way, I actually learned something. Additionally, it did
not give me any code.
 */
fn part2() -> usize {
    let red_tiles: Vec<Point> = read_lines("../aoc-inputs-2025/day9.txt")
        .expect("Cannot read file")
        .map_while(Result::ok)
        .filter_map(|line| {
            line.parse()
                .ok()
        })
        .collect();

    // compress the coordinate space (to avoid computational overhead)
    let mut x_coordinates: Vec<usize> = red_tiles
        .iter()
        .map(|point| point.x)
        .collect();
    // sort
    x_coordinates.sort_unstable();
    // remove duplicates
    x_coordinates.dedup();

    // do the same for y
    let mut y_coordinates: Vec<usize> = red_tiles
        .iter()
        .map(|point| point.y)
        .collect();

    y_coordinates.sort_unstable();
    y_coordinates.dedup();

    // use a hashmap to create a mapping between the real coordinate space and the
    // compression index
    let x_coord_mappings: HashMap<usize, usize> = x_coordinates
        .iter()
        .enumerate()
        .map(|(compression_index, &x_coord)| (x_coord, compression_index))
        .collect();

    let y_coord_mappings: HashMap<usize, usize> = y_coordinates
        .iter()
        .enumerate()
        .map(|(compression_index, &y_coord)| (y_coord, compression_index))
        .collect();

    let compressed_space_width = x_coordinates.len();
    let compressed_space_height = y_coordinates.len();

    let mut grid = vec![vec![CellType::Empty; compressed_space_width]; compressed_space_height];

    //  red tiles
    for tile in &red_tiles {
        let cx = x_coord_mappings[&tile.x];
        let cy = y_coord_mappings[&tile.y];
        grid[cy][cx] = CellType::Red;
    }

    // boundary is marked in compressed space
    for i in 0..red_tiles.len() {
        let current = red_tiles[i];
        let next = red_tiles[(i + 1) % red_tiles.len()];

        let cx1 = x_coord_mappings[&current.x];
        let cy1 = y_coord_mappings[&current.y];
        let cx2 = x_coord_mappings[&next.x];
        let cy2 = y_coord_mappings[&next.y];

        // Mark all cells between them in compressed space
        if cy1 == cy2 {
            // Horizontal line
            let (start, end) = if cx1 < cx2 { (cx1, cx2) } else { (cx2, cx1) };
            for cx in start..=end {
                if grid[cy1][cx] != CellType::Red {
                    grid[cy1][cx] = CellType::Green;
                }
            }
        } else if cx1 == cx2 {
            // Vertical line
            let (start, end) = if cy1 < cy2 { (cy1, cy2) } else { (cy2, cy1) };
            for cy in start..=end {
                if grid[cy][cx1] != CellType::Red {
                    grid[cy][cx1] = CellType::Green;
                }
            }
        }
    }

    // now do flood fill
    println!("Starting flood fill...");

    flood_fill_outside(&mut grid);
    println!("Flood fill complete!");

    // to make rectangle easy to query, build a 2d prefix sum (1 = valid, 0 = invalid)
    let mut prefix = vec![vec![0i32; compressed_space_width + 1]; compressed_space_height + 1];

    for y in 1..=compressed_space_height {
        for x in 1..=compressed_space_width {
            let is_valid =
                grid[y - 1][x - 1] == CellType::Red || grid[y - 1][x - 1] == CellType::Green;

            prefix[y][x] =
                (is_valid as i32) + prefix[y - 1][x] + prefix[y][x - 1] - prefix[y - 1][x - 1];
        }
    }

    // now make & check rectangles
    let mut max_area = 0;
    for i in 0..red_tiles.len() {
        if i % 50 == 0 {
            println!("Progress: {}/{} tiles", i, red_tiles.len());
        }

        for j in i + 1..red_tiles.len() {
            let p1 = red_tiles[i];
            let p2 = red_tiles[j];

            // Convert to compressed coordinates
            let cx1 = x_coord_mappings[&p1.x];
            let cy1 = y_coord_mappings[&p1.y];
            let cx2 = x_coord_mappings[&p2.x];
            let cy2 = y_coord_mappings[&p2.y];

            // Get rectangle bounds in compressed space
            let (min_cx, max_cx) = (cx1.min(cx2), cx1.max(cx2));
            let (min_cy, max_cy) = (cy1.min(cy2), cy1.max(cy2));

            // Check if rectangle is valid using prefix sum
            let rect_width = max_cx - min_cx + 1;
            let rect_height = max_cy - min_cy + 1;
            let expected_count = rect_width * rect_height;
            let actual_count = rect_sum(&prefix, min_cx, min_cy, max_cx, max_cy);

            if actual_count == expected_count as i32 {
                // move to real coordinates for valid rectangles
                let real_width = x_coordinates[max_cx] - x_coordinates[min_cx] + 1;
                let real_height = y_coordinates[max_cy] - y_coordinates[min_cy] + 1;
                let area = real_width * real_height;

                max_area = max_area.max(area);
            }
        }
    }

    max_area
}

fn flood_fill_outside(grid: &mut Vec<Vec<CellType>>) {
    let height = grid.len();
    let width = grid[0].len();
    let mut queue = VecDeque::new();

    // Start from all edge cells that are empty
    for y in 0..height {
        for x in 0..width {
            if (x == 0 || x == width - 1 || y == 0 || y == height - 1)
                && grid[y][x] == CellType::Empty
            {
                queue.push_back((x, y));
                grid[y][x] = CellType::Outside;
            }
        }
    }

    // mark all reachable outside cells (bfs)
    while let Some((x, y)) = queue.pop_front() {
        // Check 4 neighbors
        for (dx, dy) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;

            if nx >= 0 && nx < width as i32 && ny >= 0 && ny < height as i32 {
                let nx = nx as usize;
                let ny = ny as usize;

                if grid[ny][nx] == CellType::Empty {
                    grid[ny][nx] = CellType::Outside;
                    queue.push_back((nx, ny));
                }
            }
        }
    }

    // at this point, the empty cells are on the inside (green)
    for y in 0..height {
        for x in 0..width {
            if grid[y][x] == CellType::Empty {
                grid[y][x] = CellType::Green;
            }
        }
    }
}

fn rect_sum(prefix: &Vec<Vec<i32>>, x1: usize, y1: usize, x2: usize, y2: usize) -> i32 {
    prefix[y2 + 1][x2 + 1] - prefix[y1][x2 + 1] - prefix[y2 + 1][x1] + prefix[y1][x1]
}

fn main() {
    println!("{}", part1());
    println!("{}", part2());
}
