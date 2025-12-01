const TEST_INPUT: &str = include_str!("../../inputs/day01/test.txt");
const INPUT: &str = include_str!("../../inputs/day01/input.txt");

#[derive(Debug)]
enum Direction {
    Left(i32),
    Right(i32),
}

fn main() {
    let test = find_dial(&directions(TEST_INPUT));
    assert_eq!(test, 3);
    println!("{:?}", find_dial(&directions(INPUT)));

    assert_eq!(find_dial_newmethod(&[Direction::Left(160)]), 2);
    assert_eq!(
        find_dial_newmethod(&[
            Direction::Left(50),
            Direction::Left(160),
            Direction::Left(40),
        ]),
        3
    );
    let test = find_dial_newmethod(&directions(TEST_INPUT));
    assert_eq!(test, 6);
    println!("{:?}", find_dial_newmethod(&directions(INPUT)));
}

fn directions(input: &str) -> Vec<Direction> {
    input
        .lines()
        .map(|line| {
            let (dir, num) = line.split_at(1);
            let num = num.parse().unwrap();
            match dir {
                "L" => Direction::Left(num),
                "R" => Direction::Right(num),
                _ => panic!("Invalid direction"),
            }
        })
        .collect()
}

fn find_dial(directions: &[Direction]) -> i32 {
    let mut dial_pos = 50;
    let mut count = 0;
    for dir in directions {
        dial_pos = match dir {
            Direction::Left(n) => (dial_pos - n) % 100,
            Direction::Right(n) => (dial_pos + n) % 100,
        };
        if dial_pos == 0 {
            count += 1;
        }
    }
    count
}

fn find_dial_newmethod(directions: &[Direction]) -> i32 {
    let mut dial_pos = 50;
    let mut count = 0;
    for dir in directions {
        dial_pos = match dir {
            Direction::Left(n) => {
                count += n / 100;
                let res = (dial_pos - n).rem_euclid(100);
                // Special edge case when we already started at 0
                if dial_pos <= n % 100 && dial_pos != 0 {
                    count += 1;
                }
                res
            }
            Direction::Right(n) => {
                count += (dial_pos + n) / 100;
                (dial_pos + n) % 100
            }
        };
    }
    count
}
