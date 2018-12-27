extern crate regex;
#[macro_use] extern crate lazy_static;
extern crate utils;

use std::collections::HashSet;
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
    z: i32,
    w: i32
}

impl FromStr for Coord {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^(-?\d+),(-?\d+),(-?\d+),(-?\d+)$").unwrap();
        }
        let caps = RE.captures(s).unwrap();
        let get = |idx| caps.get(idx).unwrap().as_str().parse::<i32>().unwrap();
        Ok(Coord { x: get(1), y: get(2), z: get(3), w: get(4) })
    }
}

fn parse_input(input: &Vec<String>) -> Vec<Coord> {
    input.iter().map(|i| i.parse::<Coord>().unwrap()).collect()
}

fn manh_dist(a: &Coord, b: &Coord) -> i32 {
    (a.x - b.x).abs() + (a.y - b.y).abs() + (a.z - b.z).abs() + (a.w - b.w).abs()
}

fn part1(input: &Vec<String>) -> usize {
    let coords = parse_input(input);

    let mut matches: Vec<HashSet<usize>> = vec![];

    for i in 0..coords.len() {
        let mut cons: HashSet<usize> = HashSet::new();
        cons.insert(i);
        for j in (i + 1)..coords.len() {
            if manh_dist(&coords[i], &coords[j]) <= 3 {
                cons.insert(j);
            }
        }
        matches.push(cons);
    }

    let mut found_intersections = true;
    while found_intersections {
        let mut next: Vec<HashSet<usize>> = vec![];
        found_intersections = false;

        for j in 0..matches.len() {

            let mut found_i_next = None;
            for k in 0..next.len() {
                if matches[j].intersection(&next[k]).next().is_some() {
                    found_i_next = Some(k);
                    break;
                }
            }

            if let Some(inxt) = found_i_next {
                for &l in matches[j].iter() {
                    next[inxt].insert(l);
                }
                found_intersections = true;
            } else {
                next.push(matches[j].to_owned());
            }
        }

        matches = next;
    }

    matches.len()
}

fn main() -> Result<(), Box<Error>> {
    measure_exec(|| {
        let result = part1(&input()?);
        println!("Part1 result: {}", result);
        Ok(())
    })?;
    Ok(())
}

fn input() -> io::Result<Vec<String>> {
    let f = File::open("src/day25/input")?;
    let f = BufReader::new(f);
    Ok(f.lines().map(|l| l.unwrap()).collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn as_input(s: &str) -> Vec<String> {
        s.split('\n').map(|s| s.trim().into()).collect()
    }

    #[test]
    fn test_part1_a() {
        let input =
            "0,0,0,0
             3,0,0,0
             0,3,0,0
             0,0,3,0
             0,0,0,3
             0,0,0,6
             9,0,0,0
             12,0,0,0";
        assert_eq!(part1(&as_input(input)), 2);
    }

    #[test]
    fn test_part1_b() {
        let input =
            "-1,2,2,0
             0,0,2,-2
             0,0,0,-2
             -1,2,0,0
             -2,-2,-2,2
             3,0,2,-1
             -1,3,2,2
             -1,0,-1,0
             0,2,1,-2
             3,0,0,0";
        assert_eq!(part1(&as_input(input)), 4);
    }

    #[test]
    fn test_part1_c() {
        let input =
            "1,-1,0,1
             2,0,-1,0
             3,2,-1,0
             0,0,3,1
             0,0,-1,-1
             2,3,-2,0
             -2,2,0,0
             2,-2,0,-1
             1,-1,0,-1
             3,2,0,2";
        assert_eq!(part1(&as_input(input)), 3);
    }

    #[test]
    fn test_part1_d() {
        let input =
            "1,-1,-1,-2
             -2,-2,0,1
             0,2,1,3
             -2,3,-2,1
             0,2,3,-2
             -1,-1,1,-2
             0,-2,-1,0
             -2,2,3,-1
             1,2,2,0
             -1,-2,0,-2";
        assert_eq!(part1(&as_input(input)), 8);
    }
}