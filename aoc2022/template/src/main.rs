fn main() -> tools::Result<()> {
    let input = process_input(tools::read_input()?);
    tools::print_result(part_one(&input), part_two(&input));
    Ok(())
}

type Input = Vec<String>;

fn process_input(input: Vec<String>) -> Input {
    input
}

fn part_one(_input: &Input) -> u32 {
    0
}
fn part_two(_input: &Input) -> u32 {
    0
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
