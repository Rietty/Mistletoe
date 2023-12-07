// Module to handle/act as a library for all my various solutions.

// Imports
#[allow(unused_imports)]
use num_traits::{one, zero, FromPrimitive, Unsigned};
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
pub fn concat_slice_to_num<T: Unsigned + FromPrimitive + PartialOrd + Clone>(nums: &[T]) -> T {
    let mut together = zero();
    let mut power: T = one();

    for num in nums.iter().rev() {
        together = together + num.clone() * power.clone();
        power = power.clone() * calculate_next_power_ten(num);
    }

    together
}

fn calculate_next_power_ten<T: Unsigned + FromPrimitive + PartialOrd>(num: &T) -> T {
    let mut power = T::from_u8(10).unwrap();
    while &power <= num {
        power = power * T::from_u8(10).unwrap();
    }
    power
}
