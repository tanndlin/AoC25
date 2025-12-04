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
        let num_needed: usize = max as usize - taken;
        let num_left = batteries.len() - i;
        let rest_digits: Vec<u8> = batteries.iter().skip(i + 1).copied().collect();
        let biggest = index_of_biggest(&rest_digits, cur as u8);

        let out_of_numbers = num_left == num_needed;
        let should_take = match biggest {
            Some((_, index)) => batteries.len() - (index + i + 1) < num_needed,
            None => true,
        };
        if out_of_numbers || should_take {
            res = res * 10 + cur;
            taken += 1;
            if taken == max as usize {
                break;
            }
        }
    }

    res
}

fn index_of_biggest(nums: &[u8], value: u8) -> Option<(u8, usize)> {
    for (index, n) in nums.iter().enumerate() {
        if *n > value {
            return Some((*n, index));
        }
    }

    None
}
