mod solutions;

use crate::solutions::{aoc1::AoC1, aoc2::AoC2, aoc3::AoC3, solution::Solution};

fn main() {
    AoC1::new().run(1);
    AoC2::new().run(2);
    AoC3::new().run(3);
}
