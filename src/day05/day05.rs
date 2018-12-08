extern crate time;
extern crate utils;

use std::collections::BTreeSet;
use std::collections::BTreeMap;
use std::error::Error;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use time::*;
use utils::*;

fn opposite_polarity(a: char, b: char) -> bool {
    a.to_ascii_uppercase() == b.to_ascii_uppercase() && a as i32 - b as i32 != 0
}

fn react(polymer: &str) -> String {
    react_iter(&mut vec![], polymer.into())
}

fn react_iter(xs: &mut Vec<char>, ys: &str) -> String {
    match (xs.len(), ys) {
        (_, "") => xs.iter().collect(),
        (0, _)  => {
            xs.push(ys.chars().next().unwrap());
            react_iter(xs, &ys[1..])
        },
        (_, _) => {
            let c1 = xs[xs.len() - 1];
            let c2 = &ys[0..1].chars().next().unwrap();
            if opposite_polarity(c1, *c2) {
                xs.pop();
                react_iter(xs, &ys[1..])
            } else {
                xs.push(ys.chars().next().unwrap());
                react_iter(xs, &ys[1..])
            }
        }
    }
}

fn part1(polymer: &str) -> i32 {
    react(polymer).len() as i32
}

fn part2(polymer: &str) -> i32 {
    let units = polymer.chars()
        .fold(BTreeMap::new(), |mut units: BTreeMap<char, BTreeSet<char>>, c| {
            units.entry(c.to_ascii_uppercase()).or_insert(BTreeSet::new()).insert(c);
            units
        });

    units.iter()
        .map(|(_, chars)| {
            let reduced_polymer: String = polymer.chars().filter(|c| !chars.contains(c)).collect();
            react(&reduced_polymer[..]).len()
        })
        .min()
        .unwrap() as i32
}

fn main() -> Result<(), Box<Error>> {
    measure_exec(|| {
        let result = part1(input()?.as_str());
        println!("Part1 result: {}", result);
        Ok(())
    })?;
    measure_exec(|| {
        let result = part2(input()?.as_str());
        println!("Part2 result: {}", result);
        Ok(())
    })?;
    Ok(())
}

fn input() -> io::Result<String> {
    let f = File::open("src/day05/input")?;
    let f = BufReader::new(f);
    Ok(f.lines().map(|l| l.unwrap()).next().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = "dabAcCaCBAcCcaDA";

    #[test]
    fn test_opposite_polarity() {
        assert_eq!(opposite_polarity('a', 'A'), true);
        assert_eq!(opposite_polarity('A', 'a'), true);
        assert_eq!(opposite_polarity('a', 'a'), false);
        assert_eq!(opposite_polarity('A', 'A'), false);
        assert_eq!(opposite_polarity('a', 'B'), false);
    }

    #[test]
    fn test_react() {
        assert_eq!(react(INPUT), "dabCBAcaDA");
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 10);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 4);
    }
}