use crate::solutions::solution::Solution;

pub struct AoC7;

impl Solution for AoC7 {
    fn new() -> Self {
        AoC7 {}
    }

    fn part1(&self, input: &str) -> u64 {
        let mut grid = parse(input);
        let mut num_splits = 0;

        let mut updates = vec![];

        for y in 0..grid.len() - 1 {
            let row = &grid[y];
            for (x, cell) in row.iter().enumerate() {
                match cell {
                    Cell::Beam | Cell::Start => {
                        // If there is nothing below, record a beam placement below
                        if y < grid.len() - 1 && grid[y + 1][x] == Cell::Empty {
                            updates.push((y + 1, x, Cell::Beam));
                        }
                    }
                    Cell::Splitter => {
                        // If there is a beam above, spawn one either side of the splitter
                        if y > 0 && grid[y - 1][x] == Cell::Beam {
                            num_splits += 1;
                            if x > 0 {
                                updates.push((y, x - 1, Cell::Beam));
                                // If there is nothing below, record a beam placement below
                                if y < grid.len() - 1 && grid[y + 1][x - 1] == Cell::Empty {
                                    updates.push((y + 1, x - 1, Cell::Beam));
                                }
                            }
                            if x < row.len() - 1 {
                                updates.push((y, x + 1, Cell::Beam));
                                // If there is nothing below, record a beam placement below
                                if y < grid.len() - 1 && grid[y + 1][x + 1] == Cell::Empty {
                                    updates.push((y + 1, x + 1, Cell::Beam));
                                }
                            }
                        }
                    }
                    Cell::Empty => {}
                }
            }
            for (y, x, new_cell) in &updates {
                grid[*y][*x] = *new_cell;
            }
        }

        num_splits
    }

    fn part2(&self, input: &str) -> u64 {
        let grid = parse(input);
        let mut dp: Vec<Vec<u64>> = grid.iter().map(|r| r.iter().map(|_| 0).collect()).collect();
        let start_index = grid[0].iter().position(|c| *c == Cell::Start).unwrap();
        dp[0][start_index] = 1;

        for y in 1..dp.len() {
            for x in 0..grid[y].len() {
                // If there is nothing blocking above, trickle down
                let above = grid[y - 1][x];
                if above != Cell::Splitter {
                    dp[y][x] = dp[y - 1][x];
                }

                // Trickle splitters to the right
                if x > 0 && grid[y][x - 1] == Cell::Splitter {
                    dp[y][x] += dp[y - 1][x - 1]
                }

                // Trickle splitters to the left
                if x < grid[0].len() - 1 && grid[y][x + 1] == Cell::Splitter {
                    dp[y][x] += dp[y - 1][x + 1]
                }
            }
        }

        // Answer is the sum of the ways to get to each bottom row cell
        dp[dp.len() - 1].iter().sum()
    }
}

#[derive(Eq, PartialEq, Clone, Copy)]
enum Cell {
    Start,
    Beam,
    Splitter,
    Empty,
}

impl Cell {
    pub fn new(c: char) -> Self {
        match c {
            'S' => Cell::Start,
            '^' => Cell::Splitter,
            '.' => Cell::Empty,
            _ => panic!("Unexpected char {}", c),
        }
    }
}

fn parse(input: &str) -> Vec<Vec<Cell>> {
    let mut ret = vec![];
    for line in input.lines() {
        ret.push(line.chars().map(Cell::new).collect());
    }

    ret
}
