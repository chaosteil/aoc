fn main() -> tools::Result<()> {
    let input = process_input(tools::read_input()?);
    tools::print_result(part_one(&input), part_two(&input));
    Ok(())
}

type Input = String;

fn process_input(input: Vec<String>) -> Input {
    input[0].clone()
}

fn find_distinct(input: &Input, num: usize) -> usize {
    for (i, c) in input
        .chars()
        .collect::<Vec<_>>()
        .windows(num)
        .map(|c| c.iter().collect::<Vec<_>>())
        .map(|mut c| {
            c.sort();
            c.dedup();
            c
        })
        .enumerate()
    {
        if c.len() == num {
            return i + num;
        }
    }
    0
}

fn part_one(input: &Input) -> usize {
    find_distinct(input, 4)
}
fn part_two(input: &Input) -> usize {
    find_distinct(input, 14)
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
