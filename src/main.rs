mod solutions;

use crate::solutions::{aoc1::AoC1, solution::Solution};

fn main() {
    let sol = AoC1::new();
    let day = 1;
    println!("Advent of Code Day 1");
    println!(
        "Sample\n\tPart 1: {}\n\tPart 2: {}",
        sol.solve1(format!("{}a", day).as_str()),
        sol.solve2(format!("{}a", day).as_str())
    );
    println!(
        "Real\n\tPart 1: {}\n\tPart 2: {}",
        sol.solve1(format!("{}b", day).as_str()),
        sol.solve2(format!("{}b", day).as_str())
    );
}
