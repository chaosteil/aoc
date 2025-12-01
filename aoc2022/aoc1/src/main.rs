fn main() -> tools::Result<()> {
    let input = process_input(tools::read_input()?);
    tools::print_result(part_one(&input), part_two(&input));
    Ok(())
}

type Input = Vec<u32>;

fn process_input(input: Vec<String>) -> Input {
    let mut elves = vec![];
    let mut current: u32 = 0;
    for i in input.iter() {
        if i.is_empty() {
            elves.push(current);
            current = 0;
            continue;
        }
        current += i.parse::<u32>().unwrap();
    }
    elves
}

fn part_one(elves: &Input) -> u32 {
    *elves.into_iter().max().unwrap()
}
fn part_two(elves: &Input) -> u32 {
    let mut elves = elves.clone();
    elves.sort();
    elves.reverse();
    elves.iter().take(3).sum()
}

#[cfg(test)]
mod tests {
    #[test]
    fn testrun() -> tools::Result<()> {
        let input = super::process_input(tools::read_testinput()?);
        tools::print_result(super::part_one(&input), super::part_two(&input));
        Ok(())
    }
}
