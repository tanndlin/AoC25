use crate::{solutions::solution::Solution, utils::util::SplitLines};
use memoize::memoize;

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
        rec(
            &grid,
            0,
            grid[0].iter().position(|c| *c == Cell::Start).unwrap(),
        )
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum Cell {
    Start,
    Beam,
    Splitter,
    Empty,
}

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let symbol = match self {
            Cell::Start => 'S',
            Cell::Beam => '|',
            Cell::Splitter => '^',
            Cell::Empty => '.',
        };
        write!(f, "{}", symbol)
    }
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
    for line in input.split_lines() {
        ret.push(line.chars().map(Cell::new).collect());
    }

    ret
}

fn print_grid(grid: &Vec<Vec<Cell>>) {
    for row in grid {
        for cell in row {
            print!("{}", cell);
        }
        println!();
    }
}

// Can ignore grid, it does not change
#[memoize(Ignore: grid)]
fn rec(grid: &Vec<Vec<Cell>>, y: usize, x: usize) -> u64 {
    if y == grid.len() - 1 {
        return 1;
    }

    let below = grid[y + 1][x];
    match below {
        Cell::Start | Cell::Beam => panic!(),
        Cell::Empty => {
            // Continue on, no choice to be made
            rec(grid, y + 1, x)
        }
        Cell::Splitter => {
            // Go left
            let left = if x > 0 { rec(grid, y + 1, x - 1) } else { 0 };

            // Go right
            let right = if x < grid[y].len() - 1 {
                rec(grid, y + 1, x + 1)
            } else {
                0
            };

            left + right
        }
    }
}
