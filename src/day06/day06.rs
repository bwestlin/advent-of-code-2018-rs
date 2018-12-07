use std::cmp;
use std::collections::HashSet;
use std::str::FromStr;
use std::error::Error;
use std::num::ParseIntError;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Coord {
    pub x: i32,
    pub y: i32
}

impl FromStr for Coord {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut sp = s.split(',').map(|s| s.trim());

        Ok(Coord {
            x: sp.next().unwrap().parse().unwrap(),
            y: sp.next().unwrap().parse().unwrap()
        })
    }
}

fn manh_dist(a: &Coord, b: &Coord) -> i32 {
    (a.x - b.x).abs() + (a.y - b.y).abs()
}

fn closest_coords(c: &Coord, coords: &Vec<Coord>) -> Vec<usize> {
    let dst = |cx| manh_dist(c, cx);
    let mut dists: Vec<_> = coords.iter().map(dst).enumerate().collect();

    dists.sort_by(|(_, a), (_, b)| a.cmp(b));
    let (_, cdst) = dists[0];

    dists.iter().take_while(|(_, d)| *d == cdst).map(|(i, _)| *i).collect()
}

fn part1(coords: &Vec<Coord>) -> i32 {
    let (mx, my) = coords.iter()
        .fold((0, 0), |(mx, my), c| (cmp::max(mx, c.x + 2), cmp::max(my, c.y + 2)));

    let mut cnt_by_idx = vec![0; coords.len()];
    let mut infinite: HashSet<usize> = HashSet::new();

    for y in 0..my {
        for x in 0..mx {
            let closest = closest_coords(&Coord { x: x, y: y }, coords);
            if closest.len() == 1 {
                cnt_by_idx[closest[0]] += 1;

                if y == 0 || x == 0 || y == my - 1 || x == mx - 1 {
                    for &c in closest.iter() {
                        infinite.insert(c);
                    }
                }
            }
        }
    }

    cnt_by_idx.iter()
        .enumerate()
        .filter(|(i, _)| !infinite.contains(&i))
        .map(|(_, &c)| c)
        .max()
        .unwrap()
}

fn part2(coords: &Vec<Coord>, max_sum: i32) -> i32 {
    let (mx, my) = coords.iter()
        .fold((0, 0), |(mx, my), c| (cmp::max(mx, c.x + 2), cmp::max(my, c.y + 2)));

    let mut within_max_sum = vec![false; (mx * my) as usize];

    for y in 0..my {
        for x in 0..mx {
            let scoord = Coord { x: x, y: y };
            let csum: i32 = coords.iter().map(|c| manh_dist(&scoord, c)).sum();
            within_max_sum[(mx * y + x) as usize] = csum < max_sum;
        }
    }

    within_max_sum.iter().filter(|&&s| s).count() as i32
}

fn main() -> Result<(), Box<Error>> {
    let result = part1(&input()?);
    println!("Part1 result: {}", result);

    let result = part2(&input()?, 10000);
    println!("Part2 result: {}", result);

    Ok(())
}

fn input() -> io::Result<Vec<Coord>> {
    let f = File::open("src/day06/input")?;
    let f = BufReader::new(f);
    Ok(f.lines().map(|l| l.unwrap().trim().parse::<Coord>().unwrap()).collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str =
       "1, 1
        1, 6
        8, 3
        3, 4
        5, 5
        8, 9";

    fn as_input(s: &str) -> Vec<Coord> {
        s.split('\n').map(|s| s.trim().parse::<Coord>().unwrap()).collect()
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&as_input(INPUT)), 17);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&as_input(INPUT), 32), 16);
    }
}