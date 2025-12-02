use crate::solutions::solution::Solution;

pub struct AoC1;

impl Solution for AoC1 {
    fn new() -> Self {
        AoC1 {}
    }

    fn part1(&self, input: &str) -> u64 {
        let rotations: Vec<Rotation> = input.lines().map(parse_rotation).collect();
        // Count number of times the dial is left on 0
        let mut value = 50;
        let mut times = 0;
        for rot in rotations {
            value += rot.amount as i16
                * match rot.direction {
                    Direction::Left => -1,
                    Direction::Right => 1,
                };

            value %= 100;
            if value == 0 {
                times += 1;
            }
        }

        times
    }

    fn part2(&self, input: &str) -> u64 {
        let rotations: Vec<Rotation> = input.lines().map(parse_rotation).collect();
        // Count number of times the dial is left on 0
        let mut value = 50;
        let mut times = 0;
        for rot in rotations {
            let dir_coef = match rot.direction {
                Direction::Left => -1i16,
                Direction::Right => 1,
            };
            for _ in 0..rot.amount {
                value += dir_coef;
                value %= 100;
                if value == 0 {
                    times += 1;
                }
            }
        }

        times
    }
}

struct Rotation {
    direction: Direction,
    amount: u16,
}

enum Direction {
    Left,
    Right,
}

fn parse_rotation(s: &str) -> Rotation {
    let (dir_char, amount_str) = s.split_at(1);
    let direction = match dir_char {
        "L" => Direction::Left,
        "R" => Direction::Right,
        _ => panic!("Invalid rotation direction: {}", dir_char),
    };
    let amount = amount_str.parse::<u16>().expect("Invalid rotation amount");
    Rotation { direction, amount }
}
