use crate::solutions::solution::Solution;

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

struct Range {
    start: u64,
    end: u64,
}

impl Range {
    pub fn new(range: &str) -> Self {
        let (start_str, end_str) = range
            .trim()
            .split_once('-')
            .expect("Invalid range format, expected 'start-end'");
        Range {
            start: start_str
                .parse()
                .unwrap_or_else(|_| panic!("Unable to parse range start: {}", start_str)),
            end: end_str
                .parse()
                .unwrap_or_else(|_| panic!("Unable to parse range end: {}", end_str)),
        }
    }
}

fn parse_ranges(input: &str) -> Vec<Range> {
    input.split_terminator(',').map(Range::new).collect()
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
