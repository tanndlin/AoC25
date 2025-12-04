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

fn solve(batteries: &[u8], max: u8) -> u64 {
    let mut taken = 0;
    let mut res = 0u64;

    for i in 0..batteries.len() {
        let cur = batteries[i] as u64;
        if i == batteries.len() - 1 {
            res = res * 10 + cur;
            break;
        }

        let num_needed: usize = max as usize - taken;
        let num_left = batteries.len() - i;
        let rest_digits: Vec<u8> = batteries.iter().skip(i + 1).copied().collect();
        let biggest = n_th_biggest(&rest_digits, num_needed);

        if num_left == num_needed || biggest.iter().all(|b| cur > *b as u64) {
            res = res * 10 + cur;
            taken += 1;
            if taken == max as usize {
                break;
            }
        }
    }

    res
}

fn n_th_biggest(nums: &[u8], num_values: usize) -> Vec<u8> {
    let mut res = vec![];
    let mut taken: Vec<bool> = nums.iter().map(|_| false).collect();

    for _ in 0..num_values {
        let mut max = 0;
        let mut winner = 0;
        for (index, n) in nums.iter().enumerate() {
            if taken[index] {
                continue;
            }

            if *n > max {
                max = *n;
                winner = index;
            }
        }

        taken[winner] = true;
        res.push(max);
    }

    res
}
