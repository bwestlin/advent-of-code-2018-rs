extern crate regex;
#[macro_use] extern crate lazy_static;

use std::cmp;
use std::collections::HashSet;
use std::error::Error;
use std::num::ParseIntError;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use std::str::FromStr;
use regex::Regex;


#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Pos {
    pub x: i32,
    pub y: i32
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Size {
    pub w: i32,
    pub h: i32
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Claim {
    pub id: i32,
    pub pos: Pos,
    pub size: Size
}

impl FromStr for Claim {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^\#(\d*) @ (\d*),(\d*): (\d*)x(\d*)$").unwrap();
        }
        let caps = RE.captures(s).unwrap();
        let get = |idx| caps.get(idx).unwrap().as_str().parse();

        Ok(Claim {
            id: get(1)?,
            pos: Pos { x: get(2)?, y: get(3)? },
            size: Size { w: get(4)?, h: get(5)? }
        })
    }
}

impl Claim {
    pub fn max_x(&self) -> i32 {
        self.pos.x + self.size.w
    }
    pub fn max_y(&self) -> i32 {
        self.pos.y + self.size.h
    }
}

fn parse_claims(input: &Vec<String>) -> Result<Vec<Claim>, ParseIntError> {
    input.iter().map(|i| i.parse()).collect()
}

fn lay_claims(claims: &Vec<Claim>) -> Vec<Vec<i32>> {
    let (max_x, max_y) = claims.iter()
        .fold((0, 0), |(mx, my), c| (cmp::max(mx, c.max_x()), cmp::max(my, c.max_y())));

    let mut grid: Vec<Vec<i32>> = vec![vec![]; (max_x * max_y) as usize];

    for c in claims {
        for y in 0..c.size.h {
            for x in 0..c.size.w {
                grid[(max_x * (c.pos.y + y) + c.pos.x + x) as usize].push(c.id);
            }
        }
    }

    grid
}

fn part1(claims: &Vec<Claim>) -> i32 {
    let grid = lay_claims(claims);
    grid.iter().filter(|g| g.len() > 1).count() as i32
}

fn part2(claims: &Vec<Claim>) -> Vec<i32> {
    let claim_ids: HashSet<i32> = claims.iter().map(|c| c.id).collect();
    let grid = lay_claims(claims);
    grid.iter()
        .fold(claim_ids, |mut claim_ids, ids| {
            if ids.len() > 1 {
                for id in ids {
                    claim_ids.remove(&id);
                }
            }
            claim_ids
        })
        .iter()
        .cloned()
        .collect()
}

fn main() -> Result<(), Box<Error>> {
    let claims = parse_claims(&input()?)?;
    let overlapp_sq_inch = part1(&claims);
    println!("Part1 overlapping square inches: {}", overlapp_sq_inch);

    let non_overlapping_claim = part2(&claims);
    println!("Part2 non overlapping claim id: {:?}", non_overlapping_claim);

    Ok(())
}

fn input() -> io::Result<Vec<String>> {
    let f = File::open("src/day03/input")?;
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
    fn test_claim() {
        let claim: Claim = "#123 @ 3,2: 5x4".parse().unwrap();
        assert_eq!(claim, Claim { id: 123, pos: Pos { x: 3, y: 2 }, size: Size { w: 5, h: 4 } });
        assert_eq!(claim.max_x(), 8);
        assert_eq!(claim.max_y(), 6);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse_claims(&as_input("#1 @ 1,3: 4x4\n #2 @ 3,1: 4x4\n #3 @ 5,5: 2x2")).unwrap()), 4);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse_claims(&as_input("#1 @ 1,3: 4x4\n #2 @ 3,1: 4x4\n #3 @ 5,5: 2x2")).unwrap()), vec![3]);
    }
}