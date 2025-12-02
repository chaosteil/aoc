const TEST_INPUT: &str = include_str!("../../inputs/day03/test.txt");
const INPUT: &str = include_str!("../../inputs/day03/input.txt");

struct Bank {
    batteries: Vec<u8>,
}

fn main() {
    assert_eq!(count_two_jolts(&parse_banks(TEST_INPUT)), 357);
    assert_eq!(count_twelve_jolts(&parse_banks(TEST_INPUT)), 3121910778619);

    println!("{:?}", count_two_jolts(&parse_banks(INPUT)));
    println!("{:?}", count_twelve_jolts(&parse_banks(INPUT)));
}

fn parse_banks(input: &str) -> Vec<Bank> {
    input
        .lines()
        .map(|line| Bank {
            batteries: line.chars().map(|c| c as u8 - b'0').collect(),
        })
        .collect()
}

fn count_two_jolts(banks: &[Bank]) -> usize {
    banks
        .iter()
        .map(|bank| {
            let mut max = 0;
            for (i, l) in bank.batteries.iter().enumerate() {
                for r in bank.batteries.iter().skip(i + 1) {
                    max = max.max((*l * 10 + *r) as usize)
                }
            }
            max
        })
        .sum()
}

fn count_twelve_jolts(banks: &[Bank]) -> usize {
    banks
        .iter()
        .map(|bank| {
            let mut range = (0, bank.batteries.len() - 11);
            let mut sum = 0;
            for _ in 0..12 {
                let i = bank
                    .batteries
                    .iter()
                    .enumerate()
                    .skip(range.0)
                    .take(range.1 - range.0)
                    .rev()
                    .max_by_key(|x| x.1)
                    .unwrap();
                range.0 = i.0 + 1;
                range.1 += 1;
                sum = sum * 10 + *i.1 as usize;
            }
            sum
        })
        .sum()
}
