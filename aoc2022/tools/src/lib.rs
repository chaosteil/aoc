use std::{
    error::Error,
    fs::File,
    io::{self, BufRead},
    path::Path,
    result,
};

pub type Result<T> = result::Result<T, Box<dyn Error>>;

pub fn print_result(p1: impl std::fmt::Debug, p2: impl std::fmt::Debug) {
    println!(". part one: {:?}", p1);
    println!(".. part two: {:?}", p2);
}

pub fn read_input() -> Result<Vec<String>> {
    read_file("./input/input")
}

pub fn read_testinput() -> Result<Vec<String>> {
    read_file("./input/testinput")
}

pub fn read_file(path: impl AsRef<Path>) -> Result<Vec<String>> {
    let file = File::open(path)?;
    let lines = io::BufReader::new(file)
        .lines()
        .filter_map(io::Result::ok)
        .collect();
    Ok(lines)
}
