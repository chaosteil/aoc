use std::{collections::HashMap, str::FromStr};

const TEST_INPUT: &str = include_str!("../../inputs/day11/test.txt");
const TEST2_INPUT: &str = include_str!("../../inputs/day11/test2.txt");
const INPUT: &str = include_str!("../../inputs/day11/input.txt");

fn main() {
    assert_eq!(different_paths(&parse_input(TEST_INPUT)), 5);
    println!("{:?}", different_paths(&parse_input(INPUT)));

    assert_eq!(different_paths_rack(&parse_input(TEST2_INPUT)), 2);
    println!("{:?}", different_paths_rack(&parse_input(INPUT)));
}

struct Data {
    devices: HashMap<String, Vec<String>>,
}

fn parse_input(input: &str) -> Data {
    Data {
        devices: input
            .lines()
            .map(|line| {
                let (name, devices) = line.split_once(':').unwrap();
                let devices = devices
                    .trim()
                    .split(' ')
                    .map(|s| s.to_owned())
                    .collect::<Vec<_>>();
                (name.to_owned(), devices)
            })
            .collect(),
    }
}

fn find_path(input: &Data, from: &str, to: &str) -> usize {
    if from == to {
        return 1;
    }
    let mut total = 0;
    for link in &input.devices[from] {
        total += find_path(input, link, to);
    }
    total
}

fn different_paths(data: &Data) -> usize {
    find_path(data, "you", "out")
}

fn find_path_rack(input: &Data, from: &str, to: &str, memo: &mut HashMap<String, usize>) -> usize {
    if let Some(result) = memo.get(from) {
        return *result;
    }
    if from == to {
        return 1;
    }
    if from == "out" {
        return 0;
    }
    let mut total = 0;
    for link in &input.devices[from] {
        total += find_path_rack(input, link, to, memo);
    }
    memo.insert(from.to_string(), total);
    total
}

fn different_paths_rack(data: &Data) -> usize {
    [("svr", "fft"), ("fft", "dac"), ("dac", "out")]
        .iter()
        .map(|(from, to)| {
            let mut memo = HashMap::new();
            find_path_rack(data, from, to, &mut memo)
        })
        .product()
}
