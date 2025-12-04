const TEST_INPUT: &str = include_str!("../../inputs/day05/test.txt");
const INPUT: &str = include_str!("../../inputs/day05/input.txt");

type ID = u64;
#[derive(Clone, Debug)]
struct Range {
    start: ID,
    end: ID,
}

struct Data {
    ranges: Vec<Range>,
    ids: Vec<ID>,
}

fn main() {
    assert_eq!(find_fresh(&parse_input(TEST_INPUT)), 3);
    println!("{:?}", find_fresh(&parse_input(INPUT)));

    assert_eq!(find_fresh_total(&parse_input(TEST_INPUT)), 14);
    println!("{:?}", find_fresh_total(&parse_input(INPUT)));
}

fn parse_input(input: &str) -> Data {
    let (top, bottom) = input.split_once("\n\n").unwrap();
    let ranges = top
        .lines()
        .map(|line| {
            let (start, end) = line.split_once('-').unwrap();
            Range {
                start: start.parse().unwrap(),
                end: end.parse().unwrap(),
            }
        })
        .collect();

    let ids = bottom.lines().map(|line| line.parse().unwrap()).collect();

    Data { ranges, ids }
}

fn find_fresh(data: &Data) -> usize {
    let mut count = 0;
    'outer: for id in &data.ids {
        for range in &data.ranges {
            if *id >= range.start && *id <= range.end {
                count += 1;
                continue 'outer;
            }
        }
    }
    count
}

fn find_fresh_total(data: &Data) -> u64 {
    let mut ranges = data.ranges.clone();
    ranges.sort_by_key(|r| r.start);
    let mut total = 0;
    let mut current = ranges.first().unwrap().clone();
    for range in ranges.iter().skip(1) {
        if current.end >= range.start {
            current.end = range.end.max(current.end);
            continue;
        }
        total += current.end - current.start + 1;
        current = range.clone();
    }
    total += current.end - current.start + 1;
    total
}
