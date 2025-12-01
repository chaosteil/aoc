use std::{
    error::Error,
    fs::File,
    io::{self, BufRead},
};

use intcode::Intcode;

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("./input/input")?;
    let input: Vec<usize> = io::BufReader::new(file)
        .lines()
        .filter_map(Result::ok)
        .map(|x| Intcode::parse(&x))
        .next()
        .unwrap();
    println!("{:?}", part_one(&input));
    println!("{:?}", part_two(&input));
    Ok(())
}

fn part_one(input: &[usize]) -> usize {
    let mut ic = Intcode::new(input);
    ic[1] = 12;
    ic[2] = 2;
    ic.run();
    ic[0]
}
fn part_two(input: &[usize]) -> usize {
    for noun in 0..100 {
        for verb in 0..100 {
            let mut ic = Intcode::new(input);
            ic[1] = noun;
            ic[2] = verb;
            ic.run();
            if ic[0] == 19690720 {
                return 100 * noun + verb;
            }
        }
    }
    unreachable!()
}
