use std::path::Path;

pub trait Solution {
    fn new() -> Self;

    fn run(&self, day: u8) {
        println!("Advent of Code Day {}", day);
        println!(
            "Part 1 Sample: {:20} | Real:   {:20}",
            self.solve1(format!("{}a", day).as_str()),
            self.solve1(format!("{}b", day).as_str())
        );
        println!(
            "Part 2 Sample: {:20} | Real:   {:20}\n",
            self.solve2(format!("{}a", day).as_str()),
            self.solve2(format!("{}b", day).as_str())
        );
    }

    fn solve1(&self, input: &str) -> u64 {
        let filename = format!("src/inputs/{}.txt", input);
        let path = Path::new(&filename);
        let input = std::fs::read_to_string(path)
            .unwrap_or_else(|_| panic!("Failed to read input file for day {}", input));
        self.part1(&input)
    }

    fn solve2(&self, input: &str) -> u64 {
        let filename = format!("src/inputs/{}.txt", input);
        let path = Path::new(&filename);
        let input = std::fs::read_to_string(path)
            .unwrap_or_else(|_| panic!("Failed to read input file for day {}", input));
        self.part2(&input)
    }

    fn part1(&self, input: &str) -> u64;
    fn part2(&self, input: &str) -> u64;
}
