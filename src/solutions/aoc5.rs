use crate::solutions::solution::Solution;

pub struct AoC5;

impl Solution for AoC5 {
    fn new() -> Self {
        AoC5 {}
    }

    fn part1(&self, input: &str) -> u64 {
        let puzzle = parse(input);
        puzzle
            .ingredients
            .iter()
            .filter(|id| is_fresh(id, &puzzle.ranges))
            .count() as u64
    }

    fn part2(&self, input: &str) -> u64 {
        let puzzle = parse(input);
        consolidate_ranges(&puzzle.ranges)
            .iter()
            .map(|r| r.count_range())
            .sum()
    }
}

fn parse(input: &str) -> Puzzle {
    let (ranges_str, ids) = input.split_once("\r\n\r\n").expect("No delimiter found");
    Puzzle {
        ranges: ranges_str.split("\r\n").map(Range::new).collect(),
        ingredients: ids
            .split_ascii_whitespace()
            .map(|id| id.parse().unwrap())
            .collect(),
    }
}

fn is_fresh(id: &u64, ranges: &[Range]) -> bool {
    ranges.iter().any(|r| r.start <= *id && r.end >= *id)
}

fn consolidate_ranges(ranges: &[Range]) -> Vec<Range> {
    let mut ret: Vec<Range> = vec![];
    ret.extend(ranges.iter().cloned());

    loop {
        let mut changes = None;
        for (i, a) in ret.iter().enumerate() {
            if changes.is_some() {
                break;
            }

            for j in i + 1..ret.len() {
                let b = &ret[j];
                if !a.intersects(b) {
                    continue;
                }

                let new = a.expand(b);
                changes = Some((new, i, j));
                break;
            }
        }

        match changes {
            Some((new, to_change, to_remove)) => {
                ret[to_change] = new;
                ret.remove(to_remove);
            }
            None => break,
        }
    }

    ret
}

struct Puzzle {
    ranges: Vec<Range>,
    ingredients: Vec<u64>,
}

#[derive(Clone)]
struct Range {
    start: u64,
    end: u64,
}

impl Range {
    pub fn new(range: &str) -> Self {
        let (start_str, end_str) = range.split_once("-").unwrap();
        Range {
            start: start_str.parse().unwrap(),
            end: end_str.parse().unwrap(),
        }
    }

    pub fn intersects(&self, other: &Range) -> bool {
        self.start <= other.end && other.start <= self.end
    }

    pub fn count_range(&self) -> u64 {
        self.end - self.start + 1
    }

    fn expand(&self, other: &Range) -> Range {
        Range {
            start: self.start.min(other.start),
            end: self.end.max(other.end),
        }
    }
}
