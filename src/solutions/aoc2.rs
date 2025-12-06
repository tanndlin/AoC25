use std::str::FromStr;

use crate::{solutions::solution::Solution, utils::range::Range};

pub struct AoC2;

impl Solution for AoC2 {
    fn new() -> Self {
        AoC2 {}
    }

    fn part1(&self, input: &str) -> u64 {
        let ranges = parse_ranges(input);

        let mut total = 0;
        for range in ranges {
            for n in range.start..range.end + 1 {
                if is_invalid(n) {
                    total += n
                }
            }
        }

        total
    }

    fn part2(&self, input: &str) -> u64 {
        let ranges = parse_ranges(input);

        let mut total = 0;
        for range in ranges {
            for n in range.start..range.end + 1 {
                if is_repeating(n) {
                    total += n
                }
            }
        }

        total
    }
}

fn parse_ranges(input: &str) -> Vec<Range> {
    input
        .split_terminator(',')
        .map(|s| s.trim())
        .map(|s| Range::from_str(s).unwrap())
        .collect()
}

fn is_invalid(n: u64) -> bool {
    let as_str = n.to_string();
    let length = as_str.len();
    if length % 2 != 0 {
        return false;
    }

    let (first, second) = as_str.split_at(length / 2);
    first == second
}

fn is_repeating(n: u64) -> bool {
    let as_str = n.to_string();
    let length = as_str.len();
    for size in 1..length / 2 + 1 {
        if length % size != 0 {
            continue;
        }

        // Grab the first "size" digits
        let (repeated_group, rest) = as_str.split_at(size);
        let repetitions = length / size;

        if repeated_group.repeat(repetitions - 1) == rest {
            return true;
        }
    }

    false
}
