extern crate regex;
#[macro_use] extern crate lazy_static;
extern crate utils;

use std::error::Error;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use regex::Regex;
use utils::*;

#[derive(Debug)]
struct Game {
    n_players: usize,
    last_marble: u32
}

struct Marble {
    left: usize,
    value: u32,
    right: usize
}

impl Game {
    fn parse(input: &String) -> Game {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^(\d*) .* (\d*) .*$").unwrap();
        }
        let caps = RE.captures(input).unwrap();
        Game {
            n_players: caps.get(1).unwrap().as_str().parse().unwrap(),
            last_marble: caps.get(2).unwrap().as_str().parse().unwrap()
        }
    }

    fn play(&self) -> u32 {
        let &Game { n_players, last_marble } = self;
        let mut mnum = 0;
        let mut pidx = 0usize;
        let mut cidx = 0usize;
        let mut marbles: Vec<Marble> = vec![Marble { left: 0, value: 0, right: 0 }];
        marbles.reserve(last_marble as usize);
        let mut pscore = vec![0; n_players];

        while mnum <= last_marble {
            mnum += 1;
            let mut ncidx = marbles[cidx].right;

            if mnum % 23 == 0 {
                pscore[pidx] += mnum;
                ncidx = (0..7).fold(cidx, |i, _| marbles[i].left);

                let Marble { left, value, right } = marbles[ncidx];
                pscore[pidx] += value;
                marbles[left].right = right;
                marbles[right].left = left;
                ncidx = right;
            } else {
                let right = marbles[ncidx].right;
                marbles.push(Marble { left: ncidx, value: mnum, right: right });
                let midx = marbles.len() - 1;
                marbles[right].left = midx;
                marbles[ncidx].right = midx;
                ncidx = midx;
            }

            pidx = (pidx + 1) % n_players;
            cidx = ncidx;
        }

        pscore.iter().max().unwrap().to_owned()
    }
}

fn part1(input: &Vec<String>) -> Vec<u32> {
    input.iter()
        .map(Game::parse)
        .map(|g| g.play())
        .collect()
}

fn part2(input: &Vec<String>) -> Vec<u32> {
    input.iter()
        .map(Game::parse)
        .map(|g| Game { n_players: g.n_players, last_marble: g.last_marble * 100 })
        .map(|g| g.play())
        .collect()
}

fn main() -> Result<(), Box<Error>> {
    measure_exec(|| {
        let result = part1(&input()?)[0];
        println!("Part1 result: {:?}", result);
        Ok(())
    })?;
    measure_exec(|| {
        let result = part2(&input()?)[0];
        println!("Part2 result: {:?}", result);
        Ok(())
    })?;
    Ok(())
}

fn input() -> io::Result<Vec<String>> {
    let f = File::open("src/day09/input")?;
    let f = BufReader::new(f);
    Ok(f.lines().map(|l| l.unwrap()).collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str =
       "9 players; last marble is worth 25 points
        10 players; last marble is worth 1618 points
        13 players; last marble is worth 7999 points
        17 players; last marble is worth 1104 points
        21 players; last marble is worth 6111 points
        30 players; last marble is worth 5807 points";

    fn as_input(s: &str) -> Vec<String> {
        s.split('\n').map(|s| s.trim().into()).collect()
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&as_input(INPUT)), vec![32, 8317, 146373, 2764, 54718, 37305]);
    }
}