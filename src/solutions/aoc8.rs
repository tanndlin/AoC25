use std::{collections::HashMap, fmt::Display, str::FromStr};

use crate::solutions::solution::Solution;

pub struct AoC8;

impl Solution for AoC8 {
    fn new() -> Self {
        AoC8 {}
    }

    fn part1(&self, input: &str) -> u64 {
        let coords = parse(input).unwrap();
        let mut costs = get_costs(&coords);
        costs.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());

        let mut djs = DisjointSet::new(coords.len());
        let num_connections = if coords.len() == 20 { 10 } else { 1000 };
        for _ in 0..num_connections {
            let (_, (a, b)) = costs.pop().unwrap();
            if djs.find(a) == djs.find(b) {
                continue;
            }

            if djs.find(a) != djs.find(b) {
                djs.union(a, b);
            }
        }

        let mut groups: HashMap<usize, Vec<usize>> = HashMap::new();
        for index in 0..coords.len() {
            groups.entry(djs.find(index)).or_default().push(index);
        }

        let mut biggest_groups = groups.iter().collect::<Vec<_>>();
        biggest_groups.sort_by(|a, b| b.1.len().cmp(&a.1.len()));
        biggest_groups = biggest_groups.into_iter().take(3).collect::<Vec<_>>();

        biggest_groups
            .iter()
            .map(|(_, group)| group.len() as u64)
            .product()
    }

    fn part2(&self, input: &str) -> u64 {
        let coords = parse(input).unwrap();
        let mut costs = get_costs(&coords);
        costs.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());

        let mut djs = DisjointSet::new(coords.len());
        let mut last_xs = (0, 0);

        while djs.num_islands() > 1 {
            let (_, (a, b)) = costs.pop().unwrap();
            if djs.find(a) == djs.find(b) {
                continue;
            }

            if djs.find(a) != djs.find(b) {
                djs.union(a, b);
                let ax = coords[a].x;
                let bx = coords[b].x;
                last_xs = (ax, bx)
            }
        }

        last_xs.0 * last_xs.1
    }
}

struct Coords {
    x: u64,
    y: u64,
    z: u64,
}

impl Display for Coords {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl Coords {
    pub fn dist_squared(&self, other: &Coords) -> u64 {
        (self.x as i64 - other.x as i64).pow(2) as u64
            + (self.y as i64 - other.y as i64).pow(2) as u64
            + (self.z as i64 - other.z as i64).pow(2) as u64
    }
}

#[derive(Debug)]
enum CoordsParseError {
    InvalidFormat,
    ParseIntError,
}

impl FromStr for Coords {
    type Err = CoordsParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(',').collect();
        if parts.len() != 3 {
            return Err(CoordsParseError::InvalidFormat);
        }

        let x = parts[0]
            .parse::<u64>()
            .map_err(|_| CoordsParseError::ParseIntError)?;
        let y = parts[1]
            .parse::<u64>()
            .map_err(|_| CoordsParseError::ParseIntError)?;
        let z = parts[2]
            .parse::<u64>()
            .map_err(|_| CoordsParseError::ParseIntError)?;

        Ok(Coords { x, y, z })
    }
}

fn parse(input: &str) -> Result<Vec<Coords>, CoordsParseError> {
    input.lines().map(Coords::from_str).collect()
}

fn get_costs(coords: &[Coords]) -> Vec<(u64, (usize, usize))> {
    let mut ret = vec![];

    for (i, a) in coords.iter().enumerate() {
        for (j, b) in coords.iter().enumerate().skip(i + 1) {
            if i == j {
                continue;
            }

            ret.push((a.dist_squared(b), (i, j)));
        }
    }

    ret
}

struct DisjointSet {
    parent: Vec<usize>,
    rank: Vec<usize>,
    num_islands: usize,
}

impl DisjointSet {
    pub fn new(size: usize) -> Self {
        Self {
            parent: (0..size).collect(),
            rank: vec![0; size],
            num_islands: size,
        }
    }

    pub fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]); // path compression
        }
        self.parent[x]
    }

    pub fn union(&mut self, a: usize, b: usize) {
        let pa = self.find(a);
        let pb = self.find(b);
        if pa == pb {
            return;
        }

        if self.rank[pa] < self.rank[pb] {
            self.parent[pa] = pb;
        } else if self.rank[pa] > self.rank[pb] {
            self.parent[pb] = pa;
        } else {
            self.parent[pb] = pa;
            self.rank[pa] += 1;
        }

        self.num_islands -= 1;
    }

    pub fn num_islands(&self) -> usize {
        self.num_islands
    }
}
