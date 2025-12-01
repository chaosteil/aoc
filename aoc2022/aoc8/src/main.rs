use std::collections::HashSet;

fn main() -> tools::Result<()> {
    let input = process_input(tools::read_input()?);
    tools::print_result(part_one(&input), part_two(&input));
    Ok(())
}

type Input = Vec<Vec<i32>>;

fn process_input(input: Vec<String>) -> Input {
    input
        .into_iter()
        .map(|v| {
            v.chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect::<Vec<_>>()
        })
        .collect()
}

fn part_one(input: &Input) -> usize {
    let mut hs = HashSet::new();
    let width = input[0].len();
    let height = input.len();
    for y in 0..height {
        for x in 0..width {
            if x == 0 || y == 0 || x == width - 1 || y == height - 1 {
                hs.insert((x, y));
            }
        }
    }
    for y in 0..height {
        let mut last = 0;
        for x in 0..width {
            if input[y][x] > last {
                hs.insert((x, y));
                last = input[y][x];
            }
        }
        let mut last = 0;
        for x in (0..width).rev() {
            if input[y][x] > last {
                hs.insert((x, y));
                last = input[y][x];
            }
        }
    }
    for x in 0..width {
        let mut last = 0;
        for y in 0..height {
            if input[y][x] > last {
                hs.insert((x, y));
                last = input[y][x];
            }
        }
        let mut last = 0;
        for y in (0..height).rev() {
            if input[y][x] > last {
                hs.insert((x, y));
                last = input[y][x];
            }
        }
    }
    hs.len()
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
