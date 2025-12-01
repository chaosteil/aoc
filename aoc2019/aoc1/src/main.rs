use std::{
    error::Error,
    fs::File,
    io::{self, BufRead},
};

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("./input/input")?;
    let lines: Vec<i32> = io::BufReader::new(file)
        .lines()
        .filter_map(Result::ok)
        .map(|x| x.parse::<i32>())
        .filter_map(Result::ok)
        .collect();
    println!("{:?}", part_one(&lines));
    println!("{:?}", part_two(&lines));
    Ok(())
}

fn part_one(input: &[i32]) -> i32 {
    input.iter().map(|i| i / 3 - 2).sum()
}
fn part_two(input: &[i32]) -> i32 {
    input
        .iter()
        .map(|&(mut i)| {
            let mut value = 0;
            while i / 3 - 2 > 0 {
                i = i / 3 - 2;
                value += i;
            }
            value
        })
        .sum()
}
