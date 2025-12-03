use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

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

pub mod akal_reader {
    use std::fs::File;
    use std::io::{self, BufRead};
    use std::path::Path;

    // from:https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
    // The output is wrapped in a Result to allow matching on errors.
    // Returns an Iterator to the Reader of the lines of the file.
    pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where
        P: AsRef<Path>,
    {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
    }
}
