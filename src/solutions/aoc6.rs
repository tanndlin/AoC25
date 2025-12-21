use crate::solutions::solution::Solution;

pub struct AoC6;

impl Solution for AoC6 {
    fn new() -> Self {
        AoC6 {}
    }

    fn part1(&self, input: &str) -> u64 {
        let mut columns = vec![];
        let lines: Vec<&str> = input.lines().collect();
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

    // Hardest part of this problem is parsing and transposing the digits
    fn part2(&self, input: &str) -> u64 {
        let lines: Vec<&str> = input.lines().collect();
        let num_rows = lines.len();
        let mut transposed = vec![];

        let mut index = 0;
        let mut operators = vec![];
        loop {
            let mut nums = vec![];
            operators.push(lines[num_rows - 1].chars().nth(index).unwrap());

            // Find each index where all columns are whitespace
            while index < lines[0].len()
                && !lines
                    .iter()
                    .all(|line| line.chars().nth(index).unwrap() == ' ')
            {
                // Transpose by combining the nth digit of each number into a new number
                let number = lines[..num_rows - 1]
                    .iter()
                    .map(|line| line.chars().nth(index).unwrap())
                    .filter(|c| *c != ' ') // Remove rows with no digit
                    .collect::<String>() // Combine and parse to number
                    .parse()
                    .unwrap();

                nums.push(number);
                index += 1;
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
