extern crate regex;
#[macro_use] extern crate lazy_static;

use std::cmp;
use std::collections::HashSet;
use std::collections::HashMap;
use std::str::FromStr;
use std::error::Error;
use std::num::ParseIntError;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use regex::Regex;


fn part1(input: &Vec<String>) -> i32 {
    0
}

fn part2(input: &Vec<String>) -> i32 {
    0
}

fn main() -> Result<(), Box<Error>> {
    let result = part1(&input()?);
    println!("Part1 result: {}", result);

    let result = part2(&input()?);
    println!("Part2 result: {}", result);

    Ok(())
}

fn input() -> io::Result<Vec<String>> {
    let f = File::open("src/xDAYx/input")?;
    let f = BufReader::new(f);
    Ok(f.lines().map(|l| l.unwrap()).collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str =
       "";

    fn as_input(s: &str) -> Vec<String> {
        s.split('\n').map(|s| s.trim().into()).collect()
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&as_input(INPUT)), 1337);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&as_input(INPUT)), 1337);
    }
}