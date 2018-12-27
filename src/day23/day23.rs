extern crate regex;
#[macro_use] extern crate lazy_static;
extern crate utils;

use std::cmp::Ordering;
use std::cmp;
use std::collections::BinaryHeap;
use std::str::FromStr;
use std::error::Error;
use std::num::ParseIntError;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use regex::Regex;
use utils::*;

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
struct Coord {
    x: i32,
    y: i32,
    z: i32
}

impl Coord {
    fn new(x: i32, y: i32, z: i32) -> Coord {
        Coord { x: x, y: y, z: z }
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
struct Nanobot {
    pos: Coord,
    radius: i32
}

impl FromStr for Nanobot {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^.+=<\s*(-?\d+),\s*(-?\d+),\s*(-?\d+)>.+=(-?\d+)$").unwrap();
        }
        let caps = RE.captures(s).unwrap();
        let get = |idx| caps.get(idx).unwrap().as_str().parse::<i32>().unwrap();
        Ok(Nanobot { pos: Coord { x: get(1), y: get(2), z: get(3) }, radius: get(4) })
    }
}

fn manh_dist(a: &Coord, b: &Coord) -> i32 {
    (a.x - b.x).abs() + (a.y - b.y).abs() + (a.z - b.z).abs()
}

#[derive(PartialEq, Eq, Hash, Debug)]
struct Partition {
    p1: Coord,
    p2: Coord
}

impl Partition {
    fn new(p1: Coord, p2: Coord) -> Partition {
        Partition { p1: p1, p2: p2 }
    }

    // Split partition into 8 sub partitions
    fn subdivide(&self) -> Vec<Partition> {
        let Partition { p1, p2 } = self;
        let mut ret = vec![];
        let dx = (p2.x - p1.x) / 2;
        let dy = (p2.y - p1.y) / 2;
        let dz = (p2.z - p1.z) / 2;
        ret.push(Partition::new(Coord::new(p1.x, p1.y, p1.z), Coord::new(p1.x + dx, p1.y + dy, p1.z + dz)));
        ret.push(Partition::new(Coord::new(p1.x + dx, p1.y, p1.z), Coord::new(p2.x, p1.y + dy, p1.z + dz)));
        ret.push(Partition::new(Coord::new(p1.x, p1.y + dy, p1.z), Coord::new(p1.x + dx, p2.y, p1.z + dz)));
        ret.push(Partition::new(Coord::new(p1.x + dx, p1.y + dy, p1.z), Coord::new(p2.x, p2.y, p1.z + dz)));
        ret.push(Partition::new(Coord::new(p1.x, p1.y, p1.z + dz), Coord::new(p1.x + dx, p1.y + dy, p2.z)));
        ret.push(Partition::new(Coord::new(p1.x + dx, p1.y, p1.z + dz), Coord::new(p2.x, p1.y + dy, p2.z)));
        ret.push(Partition::new(Coord::new(p1.x, p1.y + dy, p1.z + dz), Coord::new(p1.x + dx, p2.y, p2.z)));
        ret.push(Partition::new(Coord::new(p1.x + dx, p1.y + dy, p1.z + dz), Coord::new(p2.x, p2.y, p2.z)));
        ret
    }

    // Check if nanobot is within range of partition
    fn nb_within_range(&self, nb: &Nanobot) -> bool {
        let Partition { p1, p2 } = self;
        // Check within
        if nb.pos.x >= p1.x && nb.pos.x <= p2.x &&
           nb.pos.y >= p1.y && nb.pos.y <= p2.y &&
           nb.pos.z >= p1.z && nb.pos.z <= p2.z {
            return true;
        }
        let nbp_x = cmp::min(cmp::max(p1.x, nb.pos.x), p2.x);
        let nbp_y = cmp::min(cmp::max(p1.y, nb.pos.y), p2.y);
        let nbp_z = cmp::min(cmp::max(p1.z, nb.pos.z), p2.z);
        // Left plane
        if manh_dist(&nb.pos, &Coord::new(p1.x, nbp_y, nbp_z)) <= nb.radius {
            return true;
        }
        // Right plane
        if manh_dist(&nb.pos, &Coord::new(p2.x, nbp_y, nbp_z)) <= nb.radius {
            return true;
        }
        // Top plane
        if manh_dist(&nb.pos, &Coord::new(nbp_x, p1.y, nbp_z)) <= nb.radius {
            return true;
        }
        // Bottom plane
        if manh_dist(&nb.pos, &Coord::new(nbp_x, p2.y, nbp_z)) <= nb.radius {
            return true;
        }
        // Front plane
        if manh_dist(&nb.pos, &Coord::new(nbp_x, nbp_y, p1.z)) <= nb.radius {
            return true;
        }
        // Back plane
        if manh_dist(&nb.pos, &Coord::new(nbp_x, nbp_y, p2.z)) <= nb.radius {
            return true;
        }
        false
    }

    fn dist(&self) -> i32 {
        manh_dist(&self.p1, &self.p2)
    }

    fn dist_origo(&self) -> i32 {
        let d1 = (self.p1.x).abs() + (self.p1.y).abs() + (self.p1.z).abs();
        let d2 = (self.p2.x).abs() + (self.p2.y).abs() + (self.p2.z).abs();
        if d1 < d2 { d1 } else { d2 }
    }
}

#[derive(Eq, Debug)]
struct QueuedPartition {
    p: Partition,
    nb_idx: Vec<usize>
}

impl Ord for QueuedPartition {
    fn cmp(&self, other: &QueuedPartition) -> Ordering {
        self.nb_idx.len().cmp(&other.nb_idx.len())
            .then(other.p.dist_origo().cmp(&self.p.dist_origo()))
    }
}

impl PartialOrd for QueuedPartition {
    fn partial_cmp(&self, other: &QueuedPartition) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for QueuedPartition {
    fn eq(&self, other: &QueuedPartition) -> bool {
        self.p == other.p && self.nb_idx == other.nb_idx
    }
}

fn parse_input(input: &Vec<String>) -> Vec<Nanobot> {
    input.iter()
        .map(|i| i.parse::<Nanobot>().unwrap())
        .collect()
}

fn part1(input: &Vec<String>) -> usize {
    let nbs = parse_input(input);
    let (lrange_idx, _) = nbs.iter()
        .enumerate()
        .fold((0, 0), |(li, lr), (i, nb)| {
            if nb.radius > lr { (i, nb.radius) } else { (li, lr) }
        });

    let lrange_nb = nbs[lrange_idx];

    nbs.iter()
        .filter(|nb| manh_dist(&lrange_nb.pos, &nb.pos) <= lrange_nb.radius)
        .count()
}

fn part2(input: &Vec<String>) -> i32 {
    let nbs = parse_input(input);
    let (min, max) = nbs.iter().skip(1)
        .fold((nbs[0].pos, nbs[0].pos), |(lmi, lmx), nb| {
            (
                Coord { x: cmp::min(lmi.x, nb.pos.x), y: cmp::min(lmi.y, nb.pos.y), z: cmp::min(lmi.z, nb.pos.z) },
                Coord { x: cmp::max(lmx.x, nb.pos.x), y: cmp::max(lmx.y, nb.pos.y), z: cmp::max(lmx.z, nb.pos.z) }
            )
        });

    let mut heap = BinaryHeap::new();
    heap.push(QueuedPartition { p: Partition::new(min, max), nb_idx: (0..nbs.len()).collect() });
    let found_coord;

    loop {
        if let Some(qp) = heap.pop() {
            // If the size per axis on the partition is 1 (3*1) we have reached the coordinate we're looking for
            if qp.p.dist() == 3 {
                found_coord = qp.p.p2;
                break;
            }

            // For every iteration subdivide the partition with the most nanobots in range
            for p in qp.p.subdivide() {
                let mut nb_idx = vec![];

                for i in &qp.nb_idx {
                    if p.nb_within_range(&nbs[*i]) {
                        nb_idx.push(*i);
                    }
                }
                heap.push(QueuedPartition { p: p, nb_idx: nb_idx });
            }
        }
    }

    #[cfg(feature = "print")] println!("found_coord={:?}", found_coord);
    manh_dist(&Coord::new(0, 0, 0), &found_coord)
}

fn main() -> Result<(), Box<Error>> {
    measure_exec(|| {
        let result = part1(&input()?);
        println!("Part1 result: {}", result);
        Ok(())
    })?;
    measure_exec(|| {
        let result = part2(&input()?);
        println!("Part2 result: {}", result);
        Ok(())
    })?;
    Ok(())
}

fn input() -> io::Result<Vec<String>> {
    let f = File::open("src/day23/input")?;
    let f = BufReader::new(f);
    Ok(f.lines().map(|l| l.unwrap()).collect())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    fn as_input(s: &str) -> Vec<String> {
        s.split('\n').map(|s| s.trim().into()).collect()
    }

    #[test]
    fn test_part1() {
        let input =
            "pos=<0,0,0>, r=4
             pos=<1,0,0>, r=1
             pos=<4,0,0>, r=3
             pos=<0,2,0>, r=1
             pos=<0,5,0>, r=3
             pos=<0,0,3>, r=1
             pos=<1,1,1>, r=1
             pos=<1,1,2>, r=1
             pos=<1,3,1>, r=1";
        assert_eq!(part1(&as_input(input)), 7);
    }

    #[test]
    fn test_subdivide() {
        assert_eq!(Partition::new(Coord::new(0, 0, 0), Coord::new(2, 2, 2)).subdivide().iter().collect::<HashSet<_>>(), vec![
            Partition::new(Coord::new(0, 0, 0), Coord::new(1, 1, 1)),
            Partition::new(Coord::new(1, 0, 0), Coord::new(2, 1, 1)),
            Partition::new(Coord::new(0, 1, 0), Coord::new(1, 2, 1)),
            Partition::new(Coord::new(1, 1, 0), Coord::new(2, 2, 1)),
            Partition::new(Coord::new(0, 0, 1), Coord::new(1, 1, 2)),
            Partition::new(Coord::new(1, 0, 1), Coord::new(2, 1, 2)),
            Partition::new(Coord::new(0, 1, 1), Coord::new(1, 2, 2)),
            Partition::new(Coord::new(1, 1, 1), Coord::new(2, 2, 2)),
        ].iter().collect::<HashSet<_>>());
    }

    #[test]
    fn test_part2() {
        let input =
            "pos=<10,12,12>, r=2
             pos=<12,14,12>, r=2
             pos=<16,12,12>, r=4
             pos=<14,14,14>, r=6
             pos=<50,50,50>, r=200
             pos=<10,10,10>, r=5";
        assert_eq!(part2(&as_input(input)), 36);
    }
}