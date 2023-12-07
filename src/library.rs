// Module to handle/act as a library for all my various solutions.

// Imports
#[allow(unused_imports)]
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


// Function to go from a generic vector of numbers to a long integer maximum value of that type else throw an error.
// So for example if you pass it a [4, 21, 4, 2] it gives back a 42142 as the value. Do so without any string conversions.
#[allow(unused)]
pub fn concat_vec_to_num<T>(vec: &[T]) -> Result<T, &'static str> 
where
    T: ToString + std::str::FromStr + Copy, // Ensuring T is Copy
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    let mut result_str = String::new();
    for &number in vec { // Dereferencing number
        result_str.push_str(&number.to_string());
    }

    result_str.parse::<T>().map_err(|_| "Failed to parse the concatenated string into a number")
}