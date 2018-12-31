extern crate regex;
#[macro_use] extern crate lazy_static;
extern crate utils;

use std::collections::HashSet;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::error::Error;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use regex::Regex;
use utils::*;

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
struct Pos {
    x: i32,
    y: i32
}

impl Pos {
    fn translate(&self, x: i32, y: i32) -> Pos {
        Pos { x: self.x + x, y: self.y + y }
    }
}

#[derive(PartialEq, Eq, Hash, Debug)]
enum WaterState {
    Resting, Flowing
}

struct Ground {
    clay: HashSet<Pos>,
    spring: Pos,
    water: HashMap<Pos, WaterState>,
    min_x: i32,
    min_y: i32,
    max_x: i32,
    max_y: i32
}

impl Ground {
    fn new(clay: HashSet<Pos>) -> Ground {
        let min_x = clay.iter().map(|p| p.x).min().unwrap() - 1;
        let min_y = clay.iter().map(|p| p.y).min().unwrap();
        let max_x = clay.iter().map(|p| p.x).max().unwrap() + 1;
        let max_y = clay.iter().map(|p| p.y).max().unwrap();

        let spring = Pos { x: 500, y: -10 };
        Ground {
            clay: clay,
            spring: spring,
            water: HashMap::new(),
            min_x: min_x,
            min_y: min_y,
            max_x: max_x,
            max_y: max_y
        }
    }

    fn is_occupied(&self, pos: &Pos) -> bool {
        self.clay.contains(pos) || self.water.get(pos).map(|ws| *ws == WaterState::Resting).unwrap_or(false)
    }

    fn within_bound(&self, p: &Pos) -> bool {
        p.x >= self.min_x && p.x <= self.max_x && p.y >= self.min_y && p.y <= self.max_y
    }

    fn set_water(&mut self, pos: Pos, ws: WaterState) {
        if self.within_bound(&pos) {
            self.water.insert(pos, ws);
        }
    }

    fn flow_downward_next(&mut self, flow: &Pos) -> (Pos, bool) {
        let mut flowed_out = false;
        let mut next_pos = *flow;
        for y in 1.. {
            let np = flow.translate(0, y);
            if np.y > self.max_y {
                flowed_out = true;
                break;
            }
            if self.is_occupied(&np) {
                break;
            }
            next_pos = np;
            self.set_water(np, WaterState::Flowing);
        }
        (next_pos, !flowed_out)
    }

    fn flow_sideways_next(&mut self, flow: &Pos) -> Vec<Pos> {
        let mut next = vec![];
        for &d in [-1, 1].iter() {
            let mut lp = *flow;
            for x in 1.. {
                let np = flow.translate(x * d, 0);
                if self.is_occupied(&np) {
                    next.push((lp, false));
                    break;
                }
                if !self.is_occupied(&np.translate(0, 1)) {
                    next.push((np, true));
                    break;
                }
                lp = np;
            }
        }

        let open_flows: Vec<_> = next.iter().filter(|(_, o)| *o).map(|(p, _)| *p).collect();
        let (fl, _) = next[0];
        let (fr, _) = next[1];
        for x in fl.x..=fr.x {
            self.set_water(
                Pos { x: x, y: flow.y },
                if open_flows.len() > 0 { WaterState::Flowing } else { WaterState::Resting }
            );
        }
        open_flows
    }

    fn fill_water(&mut self) {
        let mut flow_origin = HashMap::new();
        let mut open_flows =  VecDeque::new();
        open_flows.push_back(self.spring);

        while !open_flows.is_empty() {
            if let Some(flow) = open_flows.pop_front() {
                // Check if flow is submerged
                if self.is_occupied(&flow) {
                    if let Some(forig) = flow_origin.get(&flow) {
                        if !open_flows.contains(forig) {
                            open_flows.push_back(*forig);
                        }
                    }
                } else {
                    let (dnext, dopen) = self.flow_downward_next(&flow);
                    if dopen {
                        let nflows = self.flow_sideways_next(&dnext);
                        // If no new flow occured keep the current one
                        if nflows.len() == 0 {
                            open_flows.push_back(flow);
                        }
                        for nf in nflows {
                            // For every new flow that occurred store it's origin to be used if submerged
                            flow_origin.insert(nf, flow);
                            // Continue with new open flows
                            open_flows.push_back(nf);
                        }
                    }
                }
            }
        }
    }

    fn water_count(&self) -> usize {
        self.water.len()
    }

    fn resting_water_count(&self) -> usize {
        self.water.values().filter(|&ws| *ws == WaterState::Resting).count()
    }

    #[cfg(feature = "print")]
    fn print(&self) {
        println!("({}, {}) - ({}, {})", self.min_x, self.min_y, self.max_x, self.max_y);
        let mut w_rest = 0;
        let mut w_flow = 0;
        for y in self.min_y..=self.max_y {
            for x in self.min_x..=self.max_x {
                let p = Pos { x: x, y: y };

                let mut c = if self.clay.contains(&p) { '#' } else { '.' };
                if self.spring == p {
                    c = '+';
                }

                c = self.water.get(&p)
                    .map(|ws| {
                        match ws {
                            WaterState::Resting => { w_rest += 1; '~' },
                            WaterState::Flowing => { w_flow += 1; '|' }
                        }
                    })
                    .unwrap_or(c);

                print!("{}", c);
            }
            println!();
        }
        println!("Water, resting: {}, flowing={}, all={}", w_rest, w_flow, w_rest + w_flow);
    }
}

fn parse_input(input: &Vec<String>) -> Ground {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^(.)=(\d+), .=(\d+)..(\d+)$").unwrap();
    }
    let mut clay: HashSet<Pos> = HashSet::new();
    for l in input {
        let caps = RE.captures(l).unwrap();
        let get_c = |idx| caps.get(idx).unwrap().as_str().chars().next().unwrap();
        let get_i = |idx| caps.get(idx).unwrap().as_str().parse::<i32>().unwrap();

        let ax_a = get_c(1);
        let a = get_i(2);
        let b1 = get_i(3);
        let b2 = get_i(4);
        let (sx, mx) = if ax_a == 'x' { (a, 0) } else { (b1, 1) };
        let (sy, my) = if ax_a == 'y' { (a, 0) } else { (b1, 1) };
        for i in 0..=(b2 - b1) {
            clay.insert(Pos { x: sx + i * mx , y: sy + i * my });
        }
    }
    Ground::new(clay)
}

fn part1(input: &Vec<String>) -> usize {
    let mut ground = parse_input(input);

    ground.fill_water();
    #[cfg(feature = "print")] ground.print();

    ground.water_count()
}

fn part2(input: &Vec<String>) -> usize {
    let mut ground = parse_input(input);

    ground.fill_water();
    #[cfg(feature = "print")] ground.print();

    ground.resting_water_count()
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
    let f = File::open("src/day17/input")?;
    let f = BufReader::new(f);
    Ok(f.lines().map(|l| l.unwrap()).collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str =
       "x=495, y=2..7
        y=7, x=495..501
        x=501, y=3..7
        x=498, y=2..4
        x=506, y=1..2
        x=498, y=10..13
        x=504, y=10..13
        y=13, x=498..504";

    const INPUT_TRICKIER: &'static str =
       "x=510, y=0..2
        x=495, y=0..2
        x=498, y=2..6
        x=505, y=2..6
        y=7, x=498..505
        x=495, y=9..12
        x=498, y=9..12
        y=12, x=495..498
        x=502, y=10..15
        x=510, y=10..15
        y=15, x=502..510
        x=505, y=11..13
        x=507, y=11..13
        y=13, x=505..507";

    fn as_input(s: &str) -> Vec<String> {
        s.split('\n').map(|s| s.trim().into()).collect()
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&as_input(INPUT)), 57);
    }

    #[test]
    fn test_part1_trickier() {
        assert_eq!(part1(&as_input(INPUT_TRICKIER)), 131);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&as_input(INPUT)), 29);
    }

    #[test]
    fn test_part2_trickier() {
        assert_eq!(part2(&as_input(INPUT_TRICKIER)), 64);
    }
}