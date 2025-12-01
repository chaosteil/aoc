fn main() -> tools::Result<()> {
    let input = process_input(tools::read_input()?);
    tools::print_result(part_one(&input), part_two(&input));
    Ok(())
}

type Input = (Vec<Vec<char>>, Vec<(usize, usize, usize)>);

fn process_input(input: Vec<String>) -> Input {
    let sp = input.iter().position(|s| s.is_empty()).unwrap();
    let (crane, cmd) = input.split_at(sp);
    let mut cranes = vec![];
    for _ in 0..crane.last().unwrap().len() / 4 + 1 {
        cranes.push(vec![]);
    }
    for (i, cr) in cranes.iter_mut().enumerate() {
        let p = i * 4 + 1;
        for c in crane.iter().rev().skip(1) {
            let ch = c.chars().nth(p).unwrap();
            if ch != ' ' {
                cr.push(ch);
            }
        }
    }
    let cmd = cmd
        .iter()
        .skip(1)
        .map(|s| {
            s.split(' ')
                .filter_map(|s| s.parse::<usize>().ok())
                .collect::<Vec<_>>()
        })
        .map(|s| (s[0], s[1] - 1, s[2] - 1))
        .collect::<Vec<_>>();
    (cranes, cmd)
}

fn part_one((cranes, cmd): &Input) -> String {
    let mut cranes = cranes.clone();
    for (quantity, from, to) in cmd.iter() {
        for _ in 0..*quantity {
            let c: char = cranes[*from].pop().unwrap();
            cranes[*to].push(c);
        }
    }
    cranes
        .into_iter()
        .filter_map(|crane| crane.last().copied())
        .collect()
}
fn part_two((cranes, cmd): &Input) -> String {
    let mut cranes = cranes.clone();
    for (quantity, from, to) in cmd.iter() {
        let mut stack = vec![];
        for _ in 0..*quantity {
            stack.push(cranes[*from].pop().unwrap());
        }
        while !stack.is_empty() {
            cranes[*to].push(stack.pop().unwrap());
        }
    }
    cranes
        .into_iter()
        .filter_map(|crane| crane.last().copied())
        .collect()
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
