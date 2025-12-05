const TEST_INPUT: &str = include_str!("../../inputs/day06/test.txt");
const INPUT: &str = include_str!("../../inputs/day06/input.txt");

fn main() {
    assert_eq!(find_grand_total(&parse_input(TEST_INPUT)), 4277556);
    println!("{:?}", find_grand_total(&parse_input(INPUT)));

    assert_eq!(
        find_grand_total(&parse_input_cephalopod(TEST_INPUT)),
        3263827
    );
    println!("{:?}", find_grand_total(&parse_input_cephalopod(INPUT)))
}

#[derive(Debug)]
enum Op {
    Add,
    Mul,
}

impl From<char> for Op {
    fn from(value: char) -> Self {
        match value {
            '+' => Op::Add,
            '*' => Op::Mul,
            _ => unreachable!(),
        }
    }
}

struct Data {
    numbers: Vec<Vec<u64>>,
    ops: Vec<Op>,
}

fn parse_input(input: &str) -> Data {
    let data = input.lines().collect::<Vec<_>>();
    let mut numbers = Vec::new();
    for line in data.iter().take(data.len() - 1) {
        for (i, num) in line.split_ascii_whitespace().enumerate() {
            if numbers.len() <= i {
                numbers.push(Vec::<u64>::new());
            }
            numbers[i].push(num.parse().unwrap());
        }
    }
    let ops = data
        .last()
        .unwrap()
        .split_ascii_whitespace()
        .map(|x| x.chars().last().unwrap())
        .map(Op::from)
        .collect();
    Data { numbers, ops }
}

fn parse_input_cephalopod(input: &str) -> Data {
    let mut data = input.lines().map(String::from).collect::<Vec<String>>();
    for line in &mut data {
        line.push(' ');
    }

    let symbols = data.last().unwrap().chars().rev().collect::<String>();
    let mut symbols = symbols.split_inclusive(['+', '*']).collect::<Vec<_>>();
    symbols.reverse();
    data.pop();

    let mut numbers = Vec::new();
    let mut ops = Vec::new();
    let mut len = 0;
    for s in &symbols {
        let mut column = Vec::new();
        for i in 0..s.len() {
            let mut acc = 0;
            for l in &data {
                let digit = l.chars().nth(len + i).unwrap();
                if digit == ' ' {
                    continue;
                }
                acc *= 10;
                acc += (digit as u8 - b'0') as u64;
            }
            if acc != 0 {
                column.push(acc);
            }
        }
        numbers.push(column);
        ops.push(s.chars().last().unwrap().into());
        len += s.len();
    }
    Data { numbers, ops }
}

fn find_grand_total(data: &Data) -> u64 {
    let mut total = 0;
    for (i, num) in data.numbers.iter().enumerate() {
        total += match data.ops[i] {
            Op::Add => num.iter().sum::<u64>(),
            Op::Mul => num.iter().product::<u64>(),
        }
    }
    total
}
