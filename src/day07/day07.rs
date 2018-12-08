extern crate regex;
#[macro_use] extern crate lazy_static;
extern crate utils;

use std::collections::HashSet;
use std::collections::BTreeSet;
use std::collections::BTreeMap;
use std::error::Error;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use regex::Regex;
use utils::*;


#[derive(PartialEq, Debug, Clone)]
pub struct Step {
    pub id: char,
    pub req: BTreeSet<char>
}

fn parse_instructions(input: &Vec<String>) -> BTreeMap<char, Step> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^.* (.) .* (.) .*$").unwrap();
    }
    let mut steps = BTreeMap::new();

    for i in input.iter() {
        let caps = RE.captures(i).unwrap();
        let get = |idx| caps.get(idx).unwrap().as_str().chars().next().unwrap();

        let rid = get(1);
        let id = get(2);

        steps.entry(rid).or_insert(Step { id: rid, req: BTreeSet::new() });
        let Step { id: _, req } = steps.entry(id).or_insert(Step { id: id, req: BTreeSet::new() });
        req.insert(rid);
    }
    steps
}

fn part1(input: &Vec<String>) -> String {
    let steps = parse_instructions(input);
    let mut visited: HashSet<char> = HashSet::new();
    let mut result: Vec<char> = vec![];

    loop {
        let no_req = steps.iter()
            .filter(|(_, s)| {
                (*s).req.iter().filter(|r| !visited.contains(r)).count() == 0
            })
            .map(|(id, _)| id.to_owned())
            .filter(|id| !visited.contains(id))
            .next();

        match no_req {
            None => break,
            Some(s) => {
                visited.insert(s);
                result.push(s);
            }
        }
    }

    result.iter().collect()
}

fn part2(input: &Vec<String>, n_workers: usize, base_time: i32) -> i32 {
    let steps = parse_instructions(input);
    let mut workers: Vec<(char, i32)> = vec![(' ', 0); n_workers];
    let mut visited: HashSet<char> = HashSet::new();
    let mut result: Vec<char> = vec![];
    let mut tot_time = 0;

    loop {
        // Perform work
        let t = workers.iter().map(|(_, t)| t.to_owned()).filter(|&t| t > 0).min();
        if let Some(t) = t {
            tot_time += t;
            for i in 0..workers.len() {
                let (s, tl) = workers[i];
                if tl > 0 {
                    let rt = tl - t;
                    let mut rs = s;
                    if rt == 0 {
                        visited.insert(s);
                        result.push(s);
                        rs = ' ';
                    }
                    workers[i] = (rs, rt);
                }
            }
        }

        let worked_on: HashSet<char> = workers.iter().filter(|(_, t)| *t > 0).map(|(s, _)| *s).collect();

        let no_req: Vec<_> = steps.iter()
            .filter(|(_, s)| {
                (*s).req.iter().filter(|r| !visited.contains(r)).count() == 0
            })
            .map(|(id, _)| id.to_owned())
            .filter(|id| !visited.contains(id))
            .filter(|id| !worked_on.contains(id))
            .collect();

        if no_req.len() == 0 && worked_on.len() == 0 {
            break;
        }

        // Allocate available workers
        for s in no_req {
            let st = base_time + (s as i32 - 'A' as i32) + 1;

            let maybe_widx = workers.iter().enumerate().filter(|(_, (_, t))| *t == 0).map(|(i, _)| i).next();
            if let Some(widx) = maybe_widx {
                workers[widx] = (s, st);
            }
        }
    }

    tot_time
}

fn main() -> Result<(), Box<Error>> {
    measure_exec(|| {
        let result = part1(&input()?);
        println!("Part1 result: {}", result);
        Ok(())
    })?;
    measure_exec(|| {
        let result = part2(&input()?, 5, 60);
        println!("Part2 result: {}", result);
        Ok(())
    })?;
    Ok(())
}

fn input() -> io::Result<Vec<String>> {
    let f = File::open("src/day07/input")?;
    let f = BufReader::new(f);
    Ok(f.lines().map(|l| l.unwrap()).collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str =
       "Step C must be finished before step A can begin.
        Step C must be finished before step F can begin.
        Step A must be finished before step B can begin.
        Step A must be finished before step D can begin.
        Step B must be finished before step E can begin.
        Step D must be finished before step E can begin.
        Step F must be finished before step E can begin.";

    fn as_input(s: &str) -> Vec<String> {
        s.split('\n').map(|s| s.trim().into()).collect()
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&as_input(INPUT)), "CABDFE");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&as_input(INPUT), 2, 0), 15);
    }
}