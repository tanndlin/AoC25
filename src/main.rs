mod solutions;

use crate::solutions::{aoc2::AoC2, solution::Solution};

fn main() {
    let sol = AoC2::new();
    let day = 2;
    println!("Advent of Code Day {}", day);
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
