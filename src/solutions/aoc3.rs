use crate::solutions::solution::Solution;

pub struct AoC3;

impl Solution for AoC3 {
    fn new() -> Self {
        AoC3 {}
    }

    fn part1(&self, input: &str) -> u64 {
        let banks = parse(input);

        banks.iter().map(|bank| solve(bank)).sum()
    }

    fn part2(&self, input: &str) -> u64 {
        0
    }
}

fn parse(input: &str) -> Vec<Vec<u8>> {
    input
        .split_whitespace()
        .map(|line| {
            line.chars()
                .map(|c| {
                    c.to_digit(10)
                        .unwrap_or_else(|| panic!("Expected digit, got: {}", c))
                        as u8
                })
                .collect()
        })
        .collect()
}

fn solve(batteries: &[u8]) -> u64 {
    let mut max = 0u64;
    for a in 0..batteries.len() {
        for b in a + 1..batteries.len() {
            let num = batteries[a] as u64 * 10 + batteries[b] as u64;
            max = max.max(num);
        }
    }

    max
}
