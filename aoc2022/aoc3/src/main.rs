use std::collections::HashSet;

fn main() -> tools::Result<()> {
    let input = process_input(tools::read_input()?);
    tools::print_result(part_one(&input), part_two(&input));
    Ok(())
}

type Input = Vec<String>;

fn id(c: &char) -> u32 {
    let i = *c as u32;
    if i >= 97 {
        i - 96
    } else {
        i - 64 + 26
    }
}

fn process_input(input: Vec<String>) -> Input {
    input
}

fn part_one(input: &Input) -> u32 {
    input
        .iter()
        .map(|i| i.split_at(i.len() / 2))
        .map(|(left, right)| {
            let left: HashSet<char> = left.chars().collect();
            let right: HashSet<char> = right.chars().collect();
            id(left.intersection(&right).next().unwrap())
        })
        .sum()
}
fn part_two(input: &Input) -> u32 {
    input
        .iter()
        .map(|c| c.chars().collect::<HashSet<_>>())
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|s| {
            id(s[0]
                .intersection(&s[1].intersection(&s[2]).copied().collect::<HashSet<char>>())
                .next()
                .unwrap())
        })
        .sum()
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
