use aoc2025::akal_reader::read_lines;
use petgraph::unionfind::UnionFind;
use std::collections::{HashMap};
use std::str::FromStr;

// x, y, z
#[derive(Debug, PartialEq)]
struct Position(usize, usize, usize);

// dist, index, index
struct PairWithDist(usize, usize, usize);

impl FromStr for Position {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s
            .split(',')
            .collect();

        if parts.len() == 3 {
            let x_str = parts[0].trim();
            let y_str = parts[1].trim();
            let z_str = parts[2].trim();

            match (
                x_str.parse::<usize>(),
                y_str.parse::<usize>(),
                z_str.parse::<usize>(),
            ) {
                (Ok(x), Ok(y), Ok(z)) => Ok(Position(x, y, z)),
                _ => Err(format!("Invalid number format in input: {s}")),
            }
        } else {
            Err(format!("Invalid point format. Expected 'x,y', got: {s}"))
        }
    }
}
impl Ord for Position {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0
            .cmp(&other.0)
            .then(
                self.1
                    .cmp(&other.1),
            )
            .then(
                self.2
                    .cmp(&other.2),
            )
    }
}

impl PartialOrd for Position {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Position {}

impl Position {
    fn squared_distance(&self, other: &Position) -> usize {
        let dx = other
            .0
            .abs_diff(self.0);
        let dy = other
            .1
            .abs_diff(self.1);
        let dz = other
            .2
            .abs_diff(self.2);

        dx * dx + dy * dy + dz * dz
    }
}

fn num_sets_in_uf(uf: &UnionFind<usize>, num_positions: usize) -> usize {
    let mut num_sets = 0;
    
    for i in 0..num_positions {
        if uf.find(i) == i {
            num_sets += 1;
        }
    }
    num_sets

}

fn part1() -> usize {
    let positions: Vec<Position> = read_lines("../aoc-inputs-2025/day8.txt")
        .expect("Cannot read file")
        .map_while(Result::ok)
        .flat_map(|line| Position::from_str(&line))
        .collect();
    // generate all the pairs
    let mut pairs: Vec<PairWithDist> = Vec::new();
    let n = positions.len();

    for i in 0..n {
        for j in (i + 1)..n {
            let dist_sq = positions[i].squared_distance(&positions[j]);
            pairs.push(PairWithDist(dist_sq, i, j));
        }
    }

    // sort by dist
    pairs.sort_unstable_by_key(|p| p.0);

    let num_total_connections = 1000;

    // add all positions to the UnionFind (stores indices)
    // since its just indices, new with len() automatically populates all positions.
    let mut uf: UnionFind<usize> = UnionFind::new(positions.len());
    for (num_connections_made, pair) in pairs.into_iter().enumerate() {
        if num_connections_made == num_total_connections {
            break;
        }
        let PairWithDist(_, pos1, pos2) = pair;

        uf.union(pos1, pos2);
    }

    // first need a hashmap to get set sizes (root, size)
    let mut set_sizes: HashMap<usize, usize> = HashMap::new();

    for i in 0..positions.len() {
        let root = uf.find(i);
        *set_sizes
            .entry(root)
            .or_insert(0) += 1;
    }

    let mut sizes: Vec<usize> = set_sizes
        .values()
        .copied()
        .collect();
    // descending sort
    sizes.sort_unstable_by(|a, b| b.cmp(a));

    sizes
        .into_iter()
        .take(3)
        .product()
}
fn part2() -> usize {
     let positions: Vec<Position> = read_lines("../aoc-inputs-2025/day8.txt")
        .expect("Cannot read file")
        .map_while(Result::ok)
        .flat_map(|line| Position::from_str(&line))
        .collect();
    // generate all the pairs
    let mut pairs: Vec<PairWithDist> = Vec::new();
    let n = positions.len();

    for i in 0..n {
        for j in (i + 1)..n {
            let dist_sq = positions[i].squared_distance(&positions[j]);
            pairs.push(PairWithDist(dist_sq, i, j));
        }
    }

    // sort by dist
    pairs.sort_unstable_by_key(|p| p.0);

    let mut x_product = 0;


    // add all positions to the UnionFind (stores indices)
    // since its just indices, new with len() automatically populates all positions.
    let mut uf: UnionFind<usize> = UnionFind::new(positions.len());
    for pair in pairs {

        let PairWithDist(_, pos1, pos2) = pair;

        if num_sets_in_uf(&uf, n) == 2 && !uf.equiv(pos1, pos2) {
            x_product = positions[pos1].0 * positions[pos2].0;
            break;
        }
        uf.union(pos1, pos2);
    }

    x_product
}
fn main() {
    println!("{}", part1());
    println!("{}", part2());
}
