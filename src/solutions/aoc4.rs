use crate::solutions::solution::Solution;

pub struct AoC4;

impl Solution for AoC4 {
    fn new() -> Self {
        AoC4 {}
    }

    fn part1(&self, input: &str) -> u64 {
        get_accessable(&parse(input)).len() as u64
    }

    fn part2(&self, input: &str) -> u64 {
        let mut grid = parse(input);
        let mut total = 0;
        loop {
            let accessable = get_accessable(&grid);
            if accessable.is_empty() {
                break;
            }

            for (y, x) in accessable {
                total += 1;
                grid[y][x] = Cell::Empty;
            }
        }

        total
    }
}

#[derive(PartialEq, Eq)]
enum Cell {
    Empty,
    Roll,
}

impl Cell {
    pub fn new(c: char) -> Self {
        match c {
            '@' => Cell::Roll,
            _ => Cell::Empty,
        }
    }
}

fn parse(input: &str) -> Vec<Vec<Cell>> {
    input
        .split_whitespace()
        .map(|line| line.chars().map(Cell::new).collect())
        .collect()
}

fn get_accessable(grid: &[Vec<Cell>]) -> Vec<(usize, usize)> {
    let neighbors = vec![
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    let mut ret = vec![];
    for (y, row) in grid.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if *cell != Cell::Roll {
                continue;
            }

            let mut roll_neighbors = 0;
            for (dx, dy) in &neighbors {
                let new_x = x as i32 + dx;
                let new_y = y as i32 + dy;

                if new_y >= 0
                    && (new_y as usize) < grid.len()
                    && new_x >= 0
                    && (new_x as usize) < row.len()
                    && grid[new_y as usize][new_x as usize] == Cell::Roll
                {
                    roll_neighbors += 1;
                }
            }

            if roll_neighbors < 4 {
                ret.push((y, x));
            }
        }
    }

    ret
}
