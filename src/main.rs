mod solutions;
mod utils;

use crate::solutions::{
    aoc1::AoC1, aoc2::AoC2, aoc3::AoC3, aoc4::AoC4, aoc5::AoC5, aoc6::AoC6, aoc7::AoC7, aoc8::AoC8,
    aoc9::AoC9, aoc10::AoC10, aoc11::AoC11, solution::Solution,
};

fn main() {
    AoC1::new().run(1);
    AoC2::new().run(2);
    AoC3::new().run(3);
    AoC4::new().run(4);
    AoC5::new().run(5);
    AoC6::new().run(6);
    AoC7::new().run(7);
    AoC8::new().run(8);
    AoC9::new().run(9);
    AoC10::new().run(10);
    AoC11::new().run(11);
}
