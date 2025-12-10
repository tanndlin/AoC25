use std::str::FromStr;

use crate::{solutions::solution::Solution, utils::util::SplitLines};

pub struct AoC9;

impl Solution for AoC9 {
    fn new() -> Self {
        AoC9 {}
    }

    fn part1(&self, input: &str) -> u64 {
        let coords = parse(input);

        let mut max = 0;
        for (i, a) in coords.iter().enumerate() {
            for b in coords.iter().skip(i + 1) {
                let dx = (a.x - b.x).abs() + 1;
                let dy = (a.y - b.y).abs() + 1;

                max = max.max(dx * dy);
            }
        }

        max as u64
    }

    fn part2(&self, input: &str) -> u64 {
        0
    }
}

#[derive(Debug)]
struct Coords {
    x: i64,
    y: i64,
}

#[derive(Debug)]
enum CoordsParseError {
    InvalidFormat,
    ParseIntError,
}

impl FromStr for Coords {
    type Err = CoordsParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(',').collect();
        if parts.len() != 2 {
            return Err(CoordsParseError::InvalidFormat);
        }

        let x = parts[0]
            .parse::<i64>()
            .map_err(|_| CoordsParseError::ParseIntError)?;
        let y = parts[1]
            .parse::<i64>()
            .map_err(|_| CoordsParseError::ParseIntError)?;

        Ok(Coords { x, y })
    }
}

fn parse(input: &str) -> Vec<Coords> {
    input
        .split_lines()
        .map(Coords::from_str)
        .map(|r| r.unwrap())
        .collect()
}
