// This file contains all file IO related functions..
use std::{
	fs::File,
	io::{self, BufRead},
	path::Path,
};

// Read a file from a given path and return a vector of strings.
#[allow(unused)]
pub fn read_file(path: &str) -> Vec<String> {
    if let Ok(lines) = read_lines(path) {
        lines.map(|line| line.unwrap()).collect()
    } else {
        panic!("Could not read file: {path}");
    }
}

// Read lines from a file at a given file-name.
// Open file relative to the base directory of the project.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}