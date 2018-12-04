extern crate regex;
#[macro_use] extern crate lazy_static;

use std::collections::HashMap;
use std::error::Error;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use regex::Regex;


#[derive(PartialEq, Debug, Clone)]
pub struct GuardRecord {
    pub date: String,
    pub id: i32,
    pub minues_slept: u64
}

impl GuardRecord {
    pub fn count_minutes(&self) -> i32 {
        let mut cnt: i32 = 0;
        for i in 0..64 {
            cnt += (self.minues_slept >> i) as i32 & 0x1;
        }
        cnt
    }
}

fn get_input_parts(i: &String) -> (String, i32, &str) {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^\[1518-(.*) \d*:(\d*)\] (.*)$").unwrap();
    }
    let caps = RE.captures(i).unwrap();
    let get = |idx| caps.get(idx).unwrap().as_str();

    (get(1).to_string(), get(2).parse::<i32>().unwrap(), get(3))
}

fn parse_guard_records(input: &Vec<String>) -> Vec<GuardRecord> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^Guard \#(\d*) begins shift$").unwrap();
    }

    let (records, _) = input.iter()
        .fold((vec![], 0), |(mut records, mut last_min), i| {
            let (date, minute, change) = get_input_parts(i);

            if RE.is_match(change) {
                let caps = RE.captures(change).unwrap();
                let id = caps.get(1).unwrap().as_str().parse::<i32>().unwrap();
                records.push(GuardRecord { date: date, id: id, minues_slept: 0 });
                last_min = 0;
            } else {
                match change {
                    "falls asleep" => {
                        last_min = minute;
                    },
                    "wakes up" => {
                        let last_idx = records.len() - 1;
                        if let Some(last_record) = records.get_mut(last_idx) {
                            for i in last_min..minute {
                                last_record.minues_slept = last_record.minues_slept | (1 << i);
                            }
                        }
                    },
                    _ => unreachable!()
                }

            }
            (records, last_min)
        });

    records
}

fn part1(input: &Vec<String>) -> i32 {
    let guard_records = parse_guard_records(input);

    // Calculate which id has the most minutes asleep
    let (mid, _) = guard_records.iter()
        .fold(HashMap::new(), |mut acc: HashMap<i32, i32>, r| {
            *acc.entry(r.id).or_insert(0) += r.count_minutes();
            acc
        })
        .iter()
        .fold((0, 0), |(lid, lc), (id, c)| {
            if *c > lc { (*id, *c) } else { (lid, lc) }
        });

    // Calculate which minute the given id has been asleep the most
    let (mmin, _) = guard_records.iter()
        .filter(|r| r.id == mid)
        .fold([0; 60], |mut mcnt, r| {
            for i in 0..60 {
                mcnt[i as usize] += (r.minues_slept >> i) as i32 & 0x1;
            }
            mcnt
        })
        .iter()
        .enumerate()
        .fold((0 as i32, 0), |(lmin, lc), (min, c)| {
            if *c > lc { (min as i32, *c) } else { (lmin, lc) }
        });

    mid * mmin
}

fn part2(input: &Vec<String>) -> i32 {
    let guard_records = parse_guard_records(input);

    // Calculate total times slept per minute per id
    let mcnt_by_id = guard_records.iter()
        .fold(HashMap::new(), |mut acc: HashMap<i32, [i32; 64]>, r| {
            {
                let mcnt = acc.entry(r.id).or_insert([0; 64]);
                for i in 0..64 {
                    mcnt[i as usize] += (r.minues_slept >> i) as i32 & 0x1;
                }
            }
            acc
        });

    // Calculate which id has been most frequently asleep at a given minute
    let (mid, _, mmin) = mcnt_by_id.iter()
        .map(|(id, mcnt)| {
            (0..60).into_iter()
                .fold((*id, 0, 0), |(id, max, midx), i| {
                    if mcnt[i] > max { (id, mcnt[i], i as i32) } else { (id, max, midx) }
                })
        })
        .fold((0, 0, 0), |(lid, lc, li), (id, c, i)| {
            if c > lc { (id, c, i) } else { (lid, lc, li) }
        });

    mid * mmin
}

fn main() -> Result<(), Box<Error>> {
    let result = part1(&input()?);
    println!("Part1 result: {}", result);

    let result = part2(&input()?);
    println!("Part2 result: {}", result);

    Ok(())
}

fn input() -> io::Result<Vec<String>> {
    let f = File::open("src/day04/input")?;
    let f = BufReader::new(f);
    let mut lines: Vec<String> = f.lines().map(|l| l.unwrap()).collect();
    lines.sort();
    Ok(lines)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str =
       "[1518-11-01 00:00] Guard #10 begins shift
        [1518-11-01 00:05] falls asleep
        [1518-11-01 00:25] wakes up
        [1518-11-01 00:30] falls asleep
        [1518-11-01 00:55] wakes up
        [1518-11-01 23:58] Guard #99 begins shift
        [1518-11-02 00:40] falls asleep
        [1518-11-02 00:50] wakes up
        [1518-11-03 00:05] Guard #10 begins shift
        [1518-11-03 00:24] falls asleep
        [1518-11-03 00:29] wakes up
        [1518-11-04 00:02] Guard #99 begins shift
        [1518-11-04 00:36] falls asleep
        [1518-11-04 00:46] wakes up
        [1518-11-05 00:03] Guard #99 begins shift
        [1518-11-05 00:45] falls asleep
        [1518-11-05 00:55] wakes up";

    fn as_input(s: &str) -> Vec<String> {
        s.split('\n').map(|s| s.trim().into()).collect()
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&as_input(INPUT)), 240);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&as_input(INPUT)), 4455);
    }
}