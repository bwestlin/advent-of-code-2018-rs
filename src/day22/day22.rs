extern crate utils;

use std::collections::HashSet;
use std::collections::HashMap;
use std::str::FromStr;
use std::error::Error;
use std::num::ParseIntError;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use utils::*;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Pos {
    x: usize,
    y: usize
}

impl Pos {
    fn adjacent(&self) -> Vec<Pos> {
        [(-1i32, 0i32), (0, -1), (1, 0), (0, 1)].iter()
            .filter(|(mx, my)| !((self.x as i32 + mx) < 0) && !((self.y as i32 + my) < 0))
            .map(|(mx, my)| Pos { x: (self.x as i32 + mx) as usize, y: (self.y as i32 + my) as usize })
            .collect()
    }
}

impl FromStr for Pos {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let vs: Vec<_> = s.split(',').map(|s| s.trim()).collect();
        Ok(Pos {
            x: vs[0].parse::<usize>()?,
            y: vs[1].parse::<usize>()?
        })
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum RegionType {
    Rocky, Wet, Narrow
}

#[derive(Clone, Debug)]
struct Region {
    geo_idx: usize,
    er_levl: usize,
    rtype: RegionType
}

impl Region {
    fn empty() -> Region {
        Region { geo_idx: 0, er_levl: 0, rtype: RegionType::Rocky }
    }
    fn new(geo_idx: usize, er_levl: usize) -> Region {
        Region {
            geo_idx: geo_idx,
            er_levl: er_levl,
            rtype: match er_levl % 3 {
                0 => RegionType::Rocky,
                1 => RegionType::Wet,
                2 => RegionType::Narrow,
                _ => unreachable!()
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Tool {
    Torch, Gear, Neither
}

struct CaveSystem {
    target: Pos,
    regions: Vec<Vec<Region>>
}

impl CaveSystem {
    fn parse(input: &Vec<String>) -> CaveSystem {
        let depth = input[0].split(':').skip(1).map(|s| s.trim().parse::<usize>().unwrap()).next().unwrap();
        let target = input[1].split(':').skip(1).map(|s| s.trim().parse::<Pos>().unwrap()).next().unwrap();
        // Pre cslculate +1000 on both axises from target to handle part2
        let regions = CaveSystem::calc_regions(&target, &depth, 1000, 1000);
        CaveSystem { target: target, regions: regions }
    }

    fn erosion_level(geo_index: usize, depth: &usize) -> usize {
        (geo_index + depth) % 20183
    }

    fn calc_regions(target: &Pos, depth: &usize, wp: usize, hp: usize) -> Vec<Vec<Region>> {
        let w = target.x + 1 + wp;
        let h = target.y + 1 + hp;
        let mut regions: Vec<Vec<Region>> = vec![vec![Region::empty(); w]; h];

        regions[0][0] = Region::new(0, CaveSystem::erosion_level(0, depth));
        for x in 1..w {
            let gi = x * 16807;
            regions[0][x] = Region::new(gi, CaveSystem::erosion_level(gi, depth));
        }
        for y in 1..h {
            let gi = y * 48271;
            regions[y][0] = Region::new(gi, CaveSystem::erosion_level(gi, depth));
        }
        for y in 1..h {
            for x in 1..w {
                if x == target.x && y == target.y {
                    regions[y][x] = Region::new(0, CaveSystem::erosion_level(0, depth))
                } else {
                    let gi = regions[y][x - 1].er_levl * regions[y - 1][x].er_levl;
                    regions[y][x] = Region::new(gi, CaveSystem::erosion_level(gi, depth));
                }
            }
        }
        regions
    }

    fn risk_level(&self) -> usize {
        let w = self.target.x + 1;
        let h = self.target.y + 1;
        let mut rls = 0;
        for y in 0..h {
            for x in 0..w {
                rls += match self.regions[y][x].rtype {
                    RegionType::Rocky => 0,
                    RegionType::Wet => 1,
                    RegionType::Narrow => 2
                };
            }
        }
        rls
    }

    fn quickest_to_target(&mut self) -> usize {
        use Tool::*;
        use RegionType::*;

        let mut visited: HashMap<(Pos, Tool), usize> = HashMap::new();
        let mut to_eval: HashSet<(Pos, Tool, usize)> = HashSet::new();
        let start_pos = Pos { x: 0, y: 0 };
        visited.insert((start_pos, Torch), 0);
        to_eval.insert((start_pos, Torch, 0));

        let tool_valid = |rt: &RegionType, t: &Tool| {
            match (rt, t) {
                (Rocky, Neither) => false,
                (Wet, Torch) => false,
                (Narrow, Gear) => false,
                _ => true
            }
        };

        while to_eval.len() > 0 {
            let mut next_eval: HashSet<(Pos, Tool, usize)> = HashSet::new();

            { // Block needed to handle borrowing of next_eval
                let min_d = to_eval.iter().map(|(_, _, d)| d).min().unwrap();
                let eval_now: Vec<_> = to_eval.iter().filter(|(_, _, d)| *d == *min_d).collect();
                let eval_later: Vec<_> = to_eval.iter().filter(|(_, _, d)| *d != *min_d).map(|e| e.to_owned()).collect();

                for (p, t, d) in eval_now {

                    // Check if target is reached
                    if *t == Torch && p.x == self.target.x && p.y == self.target.y {
                        return *d;
                    }

                    // Add possible adjacent to later evaluation
                    for &ap in p.adjacent().iter() {
                        let r = &self.regions[ap.y][ap.x];
                        if !tool_valid(&r.rtype, t) {
                            continue;
                        }
                        let nd = *d + 1;
                        let k = (ap, *t);
                        if visited.contains_key(&k) && *visited.get(&k).unwrap() < nd {
                            continue;
                        }
                        visited.insert(k, nd);
                        next_eval.insert((ap, *t, nd));
                    }

                    // Add possible tool changes to later evaluation
                    for &nt in [Torch, Gear, Neither].iter() {
                        let r = &self.regions[p.y][p.x];
                        if nt == *t || !tool_valid(&r.rtype, &nt) {
                            continue;
                        }
                        let nd = *d + 7;
                        let k = (*p, nt);
                        if visited.contains_key(&k) && *visited.get(&k).unwrap() < nd {
                            continue;
                        }
                        visited.insert(k, nd);
                        next_eval.insert((*p, nt, nd));
                    }
                }

                for e in eval_later {
                    next_eval.insert(e);
                }
            }
            to_eval = next_eval;
        }
        unreachable!()
    }

    #[cfg(feature = "print")]
    fn print(&self) {
        use RegionType::*;
        for y in 0..self.target.y + 5 {
            for x in 0..self.target.x + 5 {
                let mut c = match self.regions[y][x].rtype {
                    Rocky => '.',
                    Wet => '=',
                    Narrow => '|'
                };
                if x == self.target.x && y == self.target.y {
                    c = 'T';
                }
                print!("{}", c);
            }
            println!();
        }
    }
}

fn part1(input: &Vec<String>) -> usize {
    let cs = CaveSystem::parse(input);
    #[cfg(feature = "print")] cs.print();
    cs.risk_level()
}

fn part2(input: &Vec<String>) -> usize {
    let mut cs = CaveSystem::parse(input);
    #[cfg(feature = "print")] cs.print();
    cs.quickest_to_target()
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
    let f = File::open("src/day22/input")?;
    let f = BufReader::new(f);
    Ok(f.lines().map(|l| l.unwrap()).collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str =
       "depth: 510
        target: 10,10";

    fn as_input(s: &str) -> Vec<String> {
        s.split('\n').map(|s| s.trim().into()).collect()
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&as_input(INPUT)), 114);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&as_input(INPUT)), 45);
    }
}
