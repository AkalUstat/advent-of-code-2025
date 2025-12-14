use aoc2025::akal_reader::read_lines;
use std::collections::{HashMap, HashSet};
use std::time::Instant;

type Node = String;
fn dfs_for_paths(
    current: Node,
    end: Node,
    adj_list: &HashMap<Node, Vec<Node>>,
    visited: &mut HashSet<Node>,
    path: &mut Vec<String>,
    all_paths: &mut Vec<Vec<String>>,
) {
    path.push(current.clone());
    visited.insert(current.clone());

    if current == end {
        all_paths.push(path.clone());
    } else if let Some(neighbors) = adj_list.get(&current) {
        for neighbor in neighbors {
            if !visited.contains(neighbor) {
                dfs_for_paths(
                    neighbor.clone(),
                    end.clone(),
                    adj_list,
                    visited,
                    path,
                    all_paths,
                );
            }
        }
    }

    path.pop();
    visited.remove(&current);
}

fn dfs_memoized(
    current: String,
    end: &str,
    graph: &HashMap<Node, Vec<Node>>,
    visited: &mut HashSet<String>,
    seen_dac: bool,
    seen_fft: bool,
    cache: &mut HashMap<(String, bool, bool), usize>,
) -> usize {
    if current == end {
        return usize::from(seen_dac && seen_fft);
    }

    // cache key (current node, seen_dac, seen_fft)
    let cache_key = (current.clone(), seen_dac, seen_fft);
    if let Some(&cached_count) = cache.get(&cache_key) {
        return cached_count;
    }

    visited.insert(current.clone());
    let new_seen_dac = seen_dac || current == "dac";
    let new_seen_fft = seen_fft || current == "fft";

    let mut count = 0;

    if let Some(neighbors) = graph.get(&current) {
        for neighbor in neighbors {
            if !visited.contains(neighbor) {
                count += dfs_memoized(
                    neighbor.clone(),
                    end,
                    graph,
                    visited,
                    new_seen_dac,
                    new_seen_fft,
                    cache,
                );
            }
        }
    }

    visited.remove(&current);

    // cache result before returning
    cache.insert(cache_key, count);

    count
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
     let time = Instant::now();
    let mut all_paths_pt1 = Vec::new();
    dfs_for_paths(
        String::from("you"),
        String::from("out"),
        &graph,
        &mut HashSet::new(),
        &mut Vec::new(),
        &mut all_paths_pt1,
    );

    let len = all_paths_pt1.len();
    println!("Part 1 in {:?}: {len}", time.elapsed());

    let mut cache = HashMap::new();
     let time = Instant::now();
    let count = dfs_memoized(
        String::from("svr"),
        "out",
        &graph,
        &mut HashSet::new(),
        false,
        false,
        &mut cache,
    );

    println!("Part 2 in {:?}: {count}", time.elapsed());
}
