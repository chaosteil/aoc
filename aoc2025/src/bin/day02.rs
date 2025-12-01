use std::collections::HashSet;

const TEST_INPUT: &str = include_str!("../../inputs/day02/test.txt");
const INPUT: &str = include_str!("../../inputs/day02/input.txt");

fn main() {
    assert_eq!(
        sum_invalid(&ranges(TEST_INPUT), find_invalid_part1),
        1227775554
    );
    println!("{:?}", sum_invalid(&ranges(INPUT), find_invalid_part1));

    assert_eq!(
        sum_invalid(
            &[ID {
                first: 1188511880,
                last: 1188511890
            }],
            find_invalid_part2
        ),
        1188511885
    );
    assert_eq!(
        sum_invalid(
            &[ID {
                first: 998,
                last: 1010
            }],
            find_invalid_part2
        ),
        2009
    );
    assert_eq!(
        sum_invalid(&ranges(TEST_INPUT), find_invalid_part2),
        4174379265
    );
    println!("{:?}", sum_invalid(&ranges(INPUT), find_invalid_part2));
}

fn digits(num: i64) -> u32 {
    num.checked_ilog10().unwrap_or(0) + 1
}

fn find_invalid_part1(id: &ID) -> Vec<i64> {
    let mut invalids = Vec::new();
    for num in id.first..=id.last {
        let digits = digits(num);
        if digits % 2 == 1 {
            continue;
        }
        if num / 10_i64.pow(digits / 2) != num % 10_i64.pow(digits / 2) {
            continue;
        }
        invalids.push(num);
    }
    invalids
}

fn splice(num: i64, start: u32, end: u32) -> i64 {
    let mut result = num / 10_i64.pow(start);
    result %= 10_i64.pow(end - start + 1);
    result
}

fn find_invalid_part2(id: &ID) -> Vec<i64> {
    let mut invalids = HashSet::new();
    for num in id.first..=id.last {
        let digits = digits(num);
        'outer: for i in 0..digits / 2 {
            if !digits.is_multiple_of(i + 1) {
                continue 'outer;
            }
            let value = splice(num, 0, i);
            for j in ((i + 1)..digits).step_by(i as usize + 1) {
                if splice(num, j, j + i) != value {
                    continue 'outer;
                }
            }
            invalids.insert(num);
        }
    }
    invalids.into_iter().collect()
}

fn sum_invalid<F>(ids: &[ID], find: F) -> i64
where
    F: Fn(&ID) -> Vec<i64>,
{
    ids.iter().map(find).map(|v| v.iter().sum::<i64>()).sum()
}

fn ranges(input: &str) -> Vec<ID> {
    input
        .split(',')
        .map(str::trim)
        .map(|line| line.split_once('-').unwrap())
        .map(|(left, right)| ID {
            first: left.parse().unwrap(),
            last: right.parse().unwrap(),
        })
        .collect()
}

struct ID {
    first: i64,
    last: i64,
}
