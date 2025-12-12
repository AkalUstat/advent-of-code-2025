use aoc2025::akal_reader::read_lines;
use std::collections::{HashMap, HashSet, VecDeque};

type Node = String;
fn dfs_for_paths(
    current: Node,
    end: Node,
    adj_list: &HashMap<Node, Vec<Node>>,
    visited: &mut HashSet<Node>,
    path: &mut Vec<String>,
        all_paths: &mut Vec<Vec<String>>
)  {
    path.push(current.clone());
    visited.insert(current.clone());

    if current == end {
        all_paths.push(path.clone());
    } else if let Some(neighbors) = adj_list.get(&current) {
        for neighbor in neighbors {
            if !visited.contains(neighbor) {
                dfs_for_paths(neighbor.clone(), end.clone(), adj_list, visited, path, all_paths);
            }
        }
    }
    
    path.pop();
    visited.remove(&current);
}



fn main() {
    let mut graph: HashMap<Node, Vec<Node>> = HashMap::new();
    read_lines("../aoc-inputs-2025/day11.txt")
        .expect("Cannot read file")
        .map_while(Result::ok)
        .for_each(|line| {
            if let Some((key, values)) = line.split_once(':') {
                let key = key.to_string();
                let neighbors: Vec<Node> = values
                    .split_whitespace()
                    .map(str::to_string)
                    .collect();

                // forward edges; not a bidirectional graph
                graph
                    .entry(key.clone())
                    .or_default()
                    .extend(
                        neighbors
                            .iter()
                            .cloned(),
                    );
            }
        });

    // part 1
    let mut all_paths_pt1 = Vec::new();
    dfs_for_paths(
        String::from("you"),
        String::from("out"),
        &graph,
        &mut HashSet::new(),
        &mut Vec::new(),
        &mut all_paths_pt1
    );

    let len = all_paths_pt1.len();
    println!("Part 1: {len}");


     let mut all_paths_pt2 = Vec::new();
    dfs_for_paths(
        String::from("svr"),
        String::from("out"),
        &graph,
        &mut HashSet::new(),
        &mut Vec::new(),
        &mut all_paths_pt2
    );

    // part 2
   let dac_fft_paths = all_paths_pt2.iter()
    .filter(|path| path.contains(&"dac".to_string()) && path.contains(&"fft".to_string()))
    .count();

    println!("Part 2: {dac_fft_paths}");

}
