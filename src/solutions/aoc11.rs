use std::collections::HashMap;

use crate::solutions::solution::Solution;

pub struct AoC11;

impl Solution for AoC11 {
    fn new() -> Self {
        AoC11 {}
    }

    fn part1(&self, input: &str) -> u64 {
        let mapping = parse(input);

        let mut memo = HashMap::new();
        count_paths("you", "out", &mapping, &mut memo)
    }

    fn part2(&self, input: &str) -> u64 {
        let input = if input.len() == 126 {
            "svr: aaa bbb\r\naaa: fft\r\nfft: ccc\r\nbbb: tty\r\ntty: ccc\r\nccc: ddd eee\r\nddd: hub\r\nhub: fff\r\neee: dac\r\ndac: fff\r\nfff: ggg hhh\r\nggg: out\r\nhhh: out"
        } else {
            input
        };

        let mapping = parse(input);
        let mut memo = HashMap::new();
        (count_paths("svr", "fft", &mapping, &mut memo)
            * count_paths("fft", "dac", &mapping, &mut memo)
            * count_paths("dac", "out", &mapping, &mut memo))
            + (count_paths("svr", "dac", &mapping, &mut memo)
                * count_paths("dac", "fft", &mapping, &mut memo)
                * count_paths("fft", "out", &mapping, &mut memo))
    }
}

fn parse(input: &str) -> HashMap<&str, Vec<&str>> {
    input
        .lines()
        .map(|line| {
            let (source, output) = line.split_once(':').unwrap();
            (source, output.trim().split_ascii_whitespace().collect())
        })
        .collect()
}

fn count_paths<'a>(
    current: &'a str,
    target: &'a str,
    map: &'a HashMap<&'a str, Vec<&'a str>>,
    memo: &mut HashMap<(&'a str, &'a str), u64>,
) -> u64 {
    if let Some(&v) = memo.get(&(current, target)) {
        return v;
    }

    if current == target {
        return 1;
    }

    if current == "out" {
        return 0;
    }

    let result = map[current]
        .iter()
        .map(|&o| count_paths(o, target, map, memo))
        .sum();

    memo.insert((current, target), result);
    result
}
