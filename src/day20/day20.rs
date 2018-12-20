extern crate utils;

use std::cmp::*;
use std::collections::HashMap;
use std::error::Error;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use utils::*;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Pos {
    x: i32,
    y: i32
}

impl Pos {
    fn transform(&self, x: i32, y: i32) -> Pos {
        Pos { x: self.x + x, y: self.y + y}
    }
}

struct DirectionsResult {
    steps: usize,
    rooms: HashMap<Pos, usize>
}

fn follow_directions(r: &[char]) -> DirectionsResult {
    let mut rooms: HashMap<Pos, usize> = HashMap::new();
    let (steps, _) = dir_iter(r, 0, Pos { x: 0, y: 0 }, &mut rooms);
    DirectionsResult {
        steps: steps,
        rooms: rooms
    }
}

fn dir_iter(dir: &[char], psc: usize, p: Pos, rooms: &mut HashMap<Pos, usize>) -> (usize, usize) {
    let mut i = 0;
    let mut scount = 0;
    let mut scounts = vec![];
    let mut lp = p;

    while i < dir.len() {
        match dir[i] {
            '(' => {
                let (s, l) = dir_iter(&dir[(i + 1)..], psc + scount, lp, rooms);
                scount += s;
                i += l + 1;
            },
            ')' => break,
            '|' => {
                scounts.push(scount);
                scount = 0;
                lp = p;
            },
            c => {
                scount += 1;
                let (tx, ty) = match c {
                    'E' => (-1,  0),
                    'W' => ( 1,  0),
                    'N' => ( 0, -1),
                    'S' => ( 0,  1),
                    _ => unreachable!()
                };
                lp = lp.transform(tx, ty);
                let d = psc + scount;
                if !rooms.contains_key(&lp) || *rooms.get(&lp).unwrap() > d {
                    rooms.insert(lp, d);
                }
            }
        }
        i += 1;
    }
    scounts.push(scount);

    let (min_s, max_s) = scounts.iter().fold((i, 0), |(mis, mas), &l| (min(mis, l), max(mas, l)));
    (if min_s == 0 { 0 } else { max_s }, i)
}

fn part1(input: &Vec<char>) -> usize {
    follow_directions(&input[..]).steps
}

fn part2(input: &Vec<char>, threshold: usize) -> usize {
    let DirectionsResult { steps: _, rooms } =  follow_directions(&input[..]);

    rooms.values().filter(|&d| *d >= threshold).count()
}

fn main() -> Result<(), Box<Error>> {
    measure_exec(|| {
        let result = part1(&input()?);
        println!("Part1 result: {}", result);
        Ok(())
    })?;
    measure_exec(|| {
        let result = part2(&input()?, 1000);
        println!("Part2 result: {}", result);
        Ok(())
    })?;
    Ok(())
}

fn input() -> io::Result<Vec<char>> {
    let f = File::open("src/day20/input")?;
    let f = BufReader::new(f);
    Ok(f.lines().next().unwrap()?.chars().skip(1).take_while(|&c| c != '$').collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn as_input(s: &str) -> Vec<char> {
        s.chars().skip(1).take_while(|&c| c != '$').collect()
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&as_input("^WNE$")), 3);
        assert_eq!(part1(&as_input("^ENWWW(NEEE|SSE(EE|N))$")), 10);
        assert_eq!(part1(&as_input("^ENNWSWW(NEWS|)SSSEEN(WNSE|)EE(SWEN|)NNN$")), 18);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&as_input("^WNE$"), 3), 1);
        assert_eq!(part2(&as_input("^WNE$"), 4), 0);
        assert_eq!(part2(&as_input("^ENWWW(NEEE|SSE(EE|N))$"), 9), 4);
        assert_eq!(part2(&as_input("^ENWWW(NEEE|SSE(EE|N))$"), 10), 1);
        assert_eq!(part2(&as_input("^ENNWSWW(NEWS|)SSSEEN(WNSE|)EE(SWEN|)NNN$"), 2), 23);
        assert_eq!(part2(&as_input("^ENNWSWW(NEWS|)SSSEEN(WNSE|)EE(SWEN|)NNN$"), 8), 17);
        assert_eq!(part2(&as_input("^ENNWSWW(NEWS|)SSSEEN(WNSE|)EE(SWEN|)NNN$"), 10), 13);
        assert_eq!(part2(&as_input("^ENNWSWW(NEWS|)SSSEEN(WNSE|)EE(SWEN|)NNN$"), 12), 11);
        assert_eq!(part2(&as_input("^ENNWSWW(NEWS|)SSSEEN(WNSE|)EE(SWEN|)NNN$"), 18), 1);
    }
}
