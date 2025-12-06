use std::collections::HashMap;

use crate::solutions::solution::Solution;

pub struct AoC6;

impl Solution for AoC6 {
    fn new() -> Self {
        AoC6 {}
    }

    fn part1(&self, input: &str) -> u64 {
        let mut columns = vec![];
        let lines: Vec<&str> = input.split("\r\n").collect();
        let (operators, numbers) = lines.split_last().unwrap();

        for _ in 0..operators.split_ascii_whitespace().collect::<Vec<_>>().len() {
            columns.push(vec![]);
        }

        for line in numbers {
            for (i, s) in line.split_ascii_whitespace().enumerate() {
                columns[i].push(s.parse().unwrap());
            }
        }

        let mut total = 0;
        for (i, op) in operators.split_ascii_whitespace().enumerate() {
            total += match op {
                "+" => columns[i].iter().sum::<u64>(),
                "*" => columns[i].iter().product::<u64>(),
                _ => panic!("Unexpected token: {}", op),
            }
        }

        total
    }

    fn part2(&self, input: &str) -> u64 {
        // let transposed = vec![];

        let lines = input.split("\r\n").collect::<Vec<_>>();
        let num_rows = lines.len();
        let mut transposed = vec![];
        // Find first index where all columns are whitespace
        let mut index = 0;
        let mut operators = vec![];
        loop {
            let mut digits = vec![];
            operators.push(lines[num_rows - 1].chars().nth(index).unwrap());

            while index < lines[0].len()
                && !lines
                    .iter()
                    .all(|line| line.chars().nth(index).unwrap() == ' ')
            {
                lines[..num_rows - 1]
                    .iter()
                    .for_each(|line| digits.push(line.chars().nth(index).unwrap()));
                index += 1;
            }

            let digits_per_number = num_rows - 1;
            let number_of_numbers = digits.len() / digits_per_number;
            let mut nums = vec![0u64; number_of_numbers];
            for i in 0..number_of_numbers {
                for j in 0..digits_per_number {
                    if let Some(new_digit) = digits[i * digits_per_number + j].to_digit(10) {
                        nums[i] *= 10;
                        nums[i] += new_digit as u64;
                    }
                }
            }

            transposed.push(nums);

            index += 1;
            if index >= lines[0].len() {
                break;
            }
        }

        let mut total = 0;
        for (i, op) in operators.iter().enumerate() {
            total += match op {
                '+' => transposed[i].iter().sum::<u64>(),
                '*' => transposed[i].iter().product::<u64>(),
                _ => panic!("Unexpected token: {}", op),
            }
        }

        total
    }
}
