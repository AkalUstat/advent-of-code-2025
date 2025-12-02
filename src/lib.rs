use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn file_reader(file_loc: &str) -> BufReader<File> {
    let file = File::open(file_loc).unwrap();
    let reader = BufReader::new(file);
    reader
}

pub fn get_files_lines(file_loc: &str) -> Vec<String> {
    let file_r: BufReader<_> = file_reader(file_loc);
    file_r
        .lines()
        .map(|l| l.unwrap())
        .filter(|s| s != &"")
        .collect()
}
