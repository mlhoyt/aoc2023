pub mod grid2d;

use std::io::{BufRead, BufReader};

pub fn read_stdin() -> Result<String, std::io::Error> {
    let file = std::io::stdin();
    let mut file = BufReader::new(file);

    let mut input = String::new();
    while file.read_line(&mut input)? > 0 {}

    Ok(input)
}
