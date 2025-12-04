use core::num;
use std::io::Cursor;

use crate::solutions::solution::Solution;

pub struct AoC3;

impl Solution for AoC3 {
    fn new() -> Self {
        AoC3 {}
    }

    fn part1(&self, input: &str) -> u64 {
        parse(input).iter().map(|bank| solve(bank, 2)).sum()
    }

    fn part2(&self, input: &str) -> u64 {
        parse(input).iter().map(|bank| solve(bank, 12)).sum()
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

fn solve(batteries: &[u8], max: usize) -> u64 {
    solve_rec(
        batteries,
        max,
        0,
        0,
        batteries.iter().map(|_| false).collect(),
    )
}

fn solve_rec(batteries: &[u8], max: usize, taken: usize, index: usize, mut mask: Vec<bool>) -> u64 {
    if taken == max {
        return mask_to_real(batteries, mask);
    }

    if index >= batteries.len() {
        return 0;
    }

    // Try take
    mask[index] = true;
    let take = solve_rec(batteries, max, taken + 1, index + 1, mask.clone());
    mask[index] = false;

    // Try don't take

    let no_take = solve_rec(batteries, max, taken, index + 1, mask);

    take.max(no_take)
}

fn mask_to_real(batteries: &[u8], mask: Vec<bool>) -> u64 {
    let mut res = 0;
    for (i, n) in batteries.iter().enumerate() {
        if mask[i] {
            res = res * 10 + *n as u64;
        }
    }

    res
}
