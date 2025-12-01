fn main() -> tools::Result<()> {
    let input = process_input(tools::read_input()?);
    tools::print_result(part_one(&input), part_two(&input));
    Ok(())
}

type Input = Vec<((u32, u32), (u32, u32))>;

fn process_input(input: Vec<String>) -> Input {
    input
        .iter()
        .map(|s| s.split(|c| [',', '-'].contains(&c)))
        .map(|v| {
            v.into_iter()
                .map(|i| i.parse::<u32>().unwrap())
                .collect::<Vec<_>>()
        })
        .map(|v| {
            let l = (v[0], v[1]);
            let r = (v[2], v[3]);
            (l, r)
        })
        .collect()
}

fn part_one(input: &Input) -> usize {
    input
        .iter()
        .filter(|(l, r)| (l.0 >= r.0 && l.1 <= r.1) || (r.0 >= l.0 && r.1 <= l.1))
        .count()
}
fn part_two(input: &Input) -> usize {
    input
        .iter()
        .filter(|(l, r)| !(l.1 < r.0 || l.0 > r.1))
        .count()
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
