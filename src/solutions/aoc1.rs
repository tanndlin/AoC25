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
            value += rot.amount as i16 * rot.direction;

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
            for _ in 0..rot.amount {
                value += rot.direction;
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
    direction: i16,
    amount: u16,
}

fn parse_rotation(s: &str) -> Rotation {
    let (dir_char, amount_str) = s.split_at(1);
    let direction = match dir_char {
        "L" => -1,
        "R" => 1,
        _ => panic!("Invalid rotation direction: {}", dir_char),
    };
    let amount = amount_str.parse::<u16>().expect("Invalid rotation amount");
    Rotation { direction, amount }
}
