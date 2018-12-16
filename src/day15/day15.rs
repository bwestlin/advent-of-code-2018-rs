extern crate utils;

use std::cmp::Ordering;
use std::collections::HashSet;
use std::collections::HashMap;
use std::error::Error;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use utils::*;

#[derive(PartialEq, Debug, Clone)]
enum UnitType {
    Elf, Goblin
}

impl UnitType {
    fn try_parse(c: char) -> Option<UnitType> {
        match c {
            'E' => Some(UnitType::Elf),
            'G' => Some(UnitType::Goblin),
            _ => None,
        }
    }

    #[cfg(feature = "print")]
    fn as_char(&self) -> char {
        match self {
            UnitType::Elf => 'E',
            UnitType::Goblin => 'G'
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
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

impl PartialOrd for Pos {
    fn partial_cmp(&self, other: &Pos) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Pos {
    fn cmp(&self, other: &Pos) -> Ordering {
        self.y.cmp(&other.y).then(self.x.cmp(&other.x))
    }
}

#[derive(Debug, Clone)]
struct Unit {
    t: UnitType,
    pos: Pos,
    pwr: i32,
    hp: i32
}

impl Unit {
    fn new(t: UnitType, x: usize, y: usize) -> Unit {
        Unit { t: t, pos: Pos { x: x, y: y }, pwr: 3, hp: 200 }
    }

    fn enemy_type(&self) -> UnitType {
        match self.t {
            UnitType::Elf => UnitType::Goblin,
            UnitType::Goblin => UnitType::Elf
        }
    }

    fn is_dead(&self) -> bool {
        self.hp <= 0
    }

    fn is_alive(&self) -> bool {
        !self.is_dead()
    }
}

#[derive(Clone)]
struct Map {
    grid: Vec<Vec<char>>,
    w: usize,
    h: usize
}

impl Map {
    fn parse(input: &Vec<String>) -> (Map, Vec<Unit>) {
        let mut grid: Vec<Vec<char>> = vec![];
        let mut units = vec![];

        for y in 0..input.len() {
            let il = &input[y];
            let mut gl: Vec<char> = vec![];
            for (x, c) in il.chars().enumerate() {
                if let Some(ut) = UnitType::try_parse(c) {
                    units.push(Unit::new(ut, x, y));
                    gl.push('.');
                } else {
                    gl.push(c);
                }
            }
            grid.push(gl);
        }

        (Map::new(grid), units)
    }

    fn new(grid: Vec<Vec<char>>) -> Map {
        let w = grid[0].len();
        let h = grid.len();
        if !grid.iter().all(|gl| gl.len() == w) {
            panic!("Grid non uniform!");
        }
        Map {
            grid: grid,
            w: w,
            h: h
        }
    }

    fn is_open(&self, x: usize, y: usize) -> bool {
        x < self.w && y < self.h && self.grid[y][x] == '.'
    }
}

#[derive(Clone)]
struct Cave<'a> {
    map: &'a Map,
    units: Vec<Unit>
}

impl<'a> Cave<'a> {
    fn enemy_target_idxs(&self, unit_idx: usize) -> Vec<usize> {
        let unit = &self.units[unit_idx];
        let enemy_type = unit.enemy_type();

        self.units.iter()
            .enumerate()
            .filter(|(_, u)| u.t == enemy_type && u.is_alive())
            .map(|(i, _)| i)
            .collect()
    }

    fn is_open(&self, x: usize, y: usize) -> bool {
        self.map.is_open(x, y) && !self.units.iter().any(|u| u.is_alive() && u.pos.x == x && u.pos.y == y)
    }

    fn enemy_inrange_pos(&self, unit_idx: usize) -> Vec<Pos> {
        self.enemy_target_idxs(unit_idx).iter()
            .map(|&i| &self.units[i])
            .flat_map(|e| {
                e.pos.adjacent().into_iter()
                    .filter(|p| self.is_open(p.x, p.y))
                    .collect::<Vec<Pos>>()
            })
            .collect()
    }

    fn enemy_reachable_pos(&self, unit_idx: usize) -> Vec<(Pos, usize)> {
        let inrange_pos = self.enemy_inrange_pos(unit_idx);
        let unit_pos = self.units[unit_idx].pos;

        let mut visited: HashMap<Pos, usize> = HashMap::new();
        visited.insert(unit_pos, 0);
        let mut to_eval = vec![unit_pos];

        let mut dst = 0;
        while to_eval.len() > 0 {
            let mut next_eval = vec![];
            for p in to_eval.iter() {
                let adj = p.adjacent();
                let ps: Vec<_> = adj.iter()
                    .filter(|p| self.is_open(p.x, p.y) && (!visited.contains_key(p) || *visited.get(p).unwrap() > dst))
                    .collect();
                for &p in ps {
                    next_eval.push(p);
                    visited.insert(p, dst);
                }
            }
            to_eval = next_eval;
            dst += 1;
        }

        inrange_pos.iter().filter(|p| visited.contains_key(p)).map(|p| (*p, *visited.get(p).unwrap())).collect()
    }

    fn enemy_choosen_pos(&self, unit_idx: usize) -> Option<Pos> {
        let reachable_pos = self.enemy_reachable_pos(unit_idx);

        reachable_pos.iter()
            .min_by(|(p1, p1d), (p2, p2d)| p1d.cmp(&p2d).then(p1.cmp(&p2)))
            .map(|(p, _)| *p)
    }

    fn enemy_choosen_min_step_dists(&self, unit_idx: usize) -> Option<(Vec<Pos>, usize)> {
        self.enemy_choosen_pos(unit_idx)
            .map(|p| self.min_distances(p, self.units[unit_idx].pos))
    }

    fn min_distances(&self, start_pos: Pos, end_pos: Pos) -> (Vec<Pos>, usize) {
        let mut visited: HashMap<Pos, usize> = HashMap::new();
        visited.insert(start_pos, 0);
        let mut to_eval = vec![start_pos];

        let mut dst = 1;
        while to_eval.len() > 0 {
            let mut next_eval = vec![];
            for p in to_eval.iter() {
                let adj = p.adjacent();
                let ps: Vec<_> = adj.iter()
                    .filter(|p| self.is_open(p.x, p.y) && (!visited.contains_key(p) || *visited.get(p).unwrap() > dst))
                    .collect();
                for &p in ps {
                    next_eval.push(p);
                    visited.insert(p, dst);
                }
            }
            to_eval = next_eval;
            dst += 1;
        }

        let adj_w_dst: Vec<_> = end_pos.adjacent().iter()
            .flat_map(|ap| visited.get(ap).map(|dst| (*ap, *dst)))
            .collect();

        let min_dst = adj_w_dst.iter().map(|(_, d)| *d).min().unwrap();
        (adj_w_dst.iter().filter(|(_, d)| *d == min_dst).map(|(ap, _)| *ap).collect(), min_dst)
    }

    fn adjacent_enemies(&self, unit_idx: usize) -> Vec<usize> {
        let unit = &self.units[unit_idx];
        let enemy_type = unit.enemy_type();
        let unit_adj: HashSet<Pos> = unit.pos.adjacent().iter().map(|p| *p).collect();

        self.units.iter()
            .enumerate()
            .filter(|(_, u)| u.t == enemy_type && u.is_alive())
            .filter(|(_, u)| unit_adj.contains(&u.pos))
            .map(|(i, _)| i)
            .collect()
    }

    fn step_unit(&mut self, unit_idx: usize) {
        let adj_enemies = self.adjacent_enemies(unit_idx);

        // Move if no adjacent enemies
        if adj_enemies.len() == 0 {
            if let Some((candidate_pos, _)) = self.enemy_choosen_min_step_dists(unit_idx) {
                self.units[unit_idx].pos = candidate_pos.iter().min().unwrap().to_owned();
            }
        }

        let adj_enemies = self.adjacent_enemies(unit_idx);

        // Attack if there are adjacent enemies
        if adj_enemies.len() > 0 {
            let &attack_idx = adj_enemies.iter()
                .min_by(|&a, &b| {
                    self.units[*a].hp.cmp(&self.units[*b].hp)
                        .then(self.units[*a].pos.cmp(&self.units[*b].pos))
                })
                .unwrap();

            self.units[attack_idx].hp -= self.units[unit_idx].pwr;
        }
    }

    fn sort_units(&mut self) {
        self.units.sort_unstable_by(|a, b| a.pos.cmp(&b.pos));
    }

    fn round(&mut self) -> bool {
        self.sort_units();

        for i in 0..self.units.len() {
            if self.units[i].is_dead() {
                continue;
            }

            // Check if any enemies left at all
            if self.enemy_target_idxs(i).len() == 0 {
                return false;
            }

            self.step_unit(i);
        }
        true
    }

    #[cfg(feature = "print")]
    fn print(&self, other_pos: &Vec<Pos>, other_c: char) {
        let Cave { map: Map { grid, w: _, h: _ }, units } = self;
        for y in 0..grid.len() {
            let gr = &grid[y];

            for x in 0..gr.len() {
                let c = units.iter()
                    .filter(|u| u.pos.y == y && u.pos.x == x && u.is_alive())
                    .map(|u| u.t.as_char())
                    .next()
                    .unwrap_or(gr[x]);

                let c = other_pos.iter()
                    .filter(|p| p.x == x && p.y == y)
                    .map(|_| other_c)
                    .next()
                    .unwrap_or(c);
                print!("{}", c);
            }
            println!();
        }
    }
}

fn part1(input: &Vec<String>) -> i32 {
    let (map, units) = Map::parse(input);
    let mut c = Cave { map: &map, units: units };

    #[cfg(feature = "print")] {
        println!("Initially:");
        c.print(&vec![], ' ');
    }

    let mut rnd = 0;
    loop {
        let rres = c.round();
        #[cfg(feature = "print")] {
            println!("After {} rounds", rnd + 1);
            c.print(&vec![], '_');
            println!("Elfs: {:?}", c.units.iter().filter(|u| u.t == UnitType::Elf && u.is_alive()).collect::<Vec<&Unit>>());
            println!("Goblins: {:?}", c.units.iter().filter(|u| u.t == UnitType::Goblin && u.is_alive()).collect::<Vec<&Unit>>());
        }

        if !rres {
            #[cfg(feature = "print")] println!("Done at round {}", rnd);
            break;
        }
        rnd += 1;
    }

    let rest_unit_hp: i32 = c.units.iter().filter(|u| u.is_alive()).map(|u| u.hp).sum();
    #[cfg(feature = "print")] {
        println!("units left: {:?}", c.units.iter().filter(|u| u.is_alive()).collect::<Vec<&Unit>>());
        println!("rest_unit_hp={}, rnd={}", rest_unit_hp, rnd);
    }
    rnd * rest_unit_hp
}

fn part2(input: &Vec<String>) -> i32 {
    let (map, units) = Map::parse(input);

    for elf_power in 4.. {
        let units = units.iter()
            .map(|u| Unit {
                t: u.t.to_owned(),
                pos: u.pos,
                pwr: if u.t == UnitType::Elf { elf_power } else { u.pwr },
                hp: u.hp
            })
            .collect();
        let mut c = Cave { map: &map, units: units };

        #[cfg(feature = "print")] {
            let num_elfs = c.units.iter().filter(|u| u.t == UnitType::Elf).count();
            println!("==============================================================");
            println!("Next elf_power={}, num_elfs={}", elf_power, num_elfs);
            println!("Initially:");
            c.print(&vec![], '?');
        }

        let mut rnd = 0;
        let mut any_killed_elf;
        loop {
            let rres = c.round();

            #[cfg(feature = "print")] {
                let num_alive_elfs = c.units.iter().filter(|u| u.t == UnitType::Elf && u.is_alive()).count();
                println!("--------------------------------------------------------------");
                println!("After {} rounds, rres={}, elf_power={}, num_alive_elfs={}", rnd + 1, rres, elf_power, num_alive_elfs);
                c.print(&vec![], '?');
                c.sort_units();
                println!("Units:");
                for u in c.units.iter() {
                    println!("  {:?}", u);
                }
            }

            any_killed_elf = c.units.iter().any(|u| u.t == UnitType::Elf && u.is_dead());

            if !rres || any_killed_elf {
                #[cfg(feature = "print")] println!("Done at round {}", rnd);
                break;
            }
            rnd += 1;
        }

        if !any_killed_elf {
            let rest_unit_hp: i32 = c.units.iter().filter(|u| u.is_alive()).map(|u| u.hp).sum();
            #[cfg(feature = "print")] {
                println!("units left: {:?}", c.units.iter().filter(|u| u.is_alive()).collect::<Vec<&Unit>>());
                println!("rest_unit_hp={}, rnd={}", rest_unit_hp, rnd);
            }
            return rnd * rest_unit_hp;
        }
    }
    0 // Non reachable
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
    let f = File::open("src/day15/input")?;
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
    fn test_step1() {
        let input =
            "#######
             #E..G.#
             #...#.#
             #.G.#G#
             #######";
        let (map, units) = Map::parse(&as_input(input));
        let mut c = Cave { map: &map, units: units };
        let ui = 0;

        #[cfg(feature = "print")] {
            println!("In range:");
            let inrange_pos = c.enemy_inrange_pos(ui);
            c.print(&inrange_pos, '?');

            println!();
            println!("Reachable:");
            let reachable_pos = c.enemy_reachable_pos(ui).iter().map(|(p, _)| *p).collect();
            c.print(&reachable_pos, '@');

            println!();
            println!("Choosen:");
            let choosen_pos = c.enemy_choosen_pos(ui).unwrap();
            c.print(&vec![choosen_pos], '+');

            println!();
            println!("Min dists:");
            let (min_dist_pos, min_dist) = c.enemy_choosen_min_step_dists(ui).unwrap();
            c.print(&min_dist_pos, ('0' as u8 + min_dist as u8) as char);
        }

        c.step_unit(ui);

        #[cfg(feature = "print")] {
            println!();
            println!("Step:");
            c.print(&vec![], '?');
        }

        assert_eq!(c.units[ui].pos, Pos { x: 2, y: 1 });
    }

    #[test]
    fn test_step2() {
        let input =
            "#######
             #.E...#
             #.....#
             #...G.#
             #######";
        let (map, units) = Map::parse(&as_input(input));
        let mut c = Cave { map: &map, units: units };
        let ui = 1;

        #[cfg(feature = "print")] {
            println!("In range:");
            let inrange_pos = c.enemy_inrange_pos(ui);
            c.print(&inrange_pos, '?');

            println!();
            println!("Reachable:");
            let reachable_pos = c.enemy_reachable_pos(ui).iter().map(|(p, _)| *p).collect();
            c.print(&reachable_pos, '@');

            println!();
            println!("Choosen:");
            let choosen_pos = c.enemy_choosen_pos(ui).unwrap();
            c.print(&vec![choosen_pos], '+');

            println!();
            println!("Min dists:");
            let (min_dist_pos, min_dist) = c.enemy_choosen_min_step_dists(ui).unwrap();
            c.print(&min_dist_pos, ('0' as u8 + min_dist as u8) as char);
        }

        c.step_unit(ui);

        #[cfg(feature = "print")] {
            println!();
            println!("Step:");
            c.print(&vec![], '?');
        }

        assert_eq!(c.units[ui].pos, Pos { x: 4, y: 2 });
    }

    #[test]
    fn test_part1_a() {
        let input =
            "#######
             #.G...#
             #...EG#
             #.#.#G#
             #..G#E#
             #.....#
             #######";
        assert_eq!(part1(&as_input(input)), 27730);
    }

    #[test]
    fn test_part1_b() {
        let input =
            "#######
             #G..#E#
             #E#E.E#
             #G.##.#
             #...#E#
             #...E.#
             #######";
        assert_eq!(part1(&as_input(input)), 36334);
    }

    #[test]
    fn test_part1_c() {
        let input =
            "#######
             #E..EG#
             #.#G.E#
             #E.##E#
             #G..#.#
             #..E#.#
             #######";
        assert_eq!(part1(&as_input(input)), 39514);
    }

    #[test]
    fn test_part1_d() {
        let input =
            "#######
             #E.G#.#
             #.#G..#
             #G.#.G#
             #G..#.#
             #...E.#
             #######";
        assert_eq!(part1(&as_input(input)), 27755);
    }

    #[test]
    fn test_part1_e() {
        let input =
            "#######
             #.E...#
             #.#..G#
             #.###.#
             #E#G#G#
             #...#G#
             #######";
        assert_eq!(part1(&as_input(input)), 28944);
    }

    #[test]
    fn test_part1_f() {
        let input =
            "#########
             #G......#
             #.E.#...#
             #..##..G#
             #...##..#
             #...#...#
             #.G...G.#
             #.....G.#
             #########";
        assert_eq!(part1(&as_input(input)), 18740);
    }

    #[test]
    fn test_part2_a() {
        let input =
            "#######
             #.G...#
             #...EG#
             #.#.#G#
             #..G#E#
             #.....#
             #######";
        assert_eq!(part2(&as_input(input)), 4988);
    }

    #[test]
    fn test_part2_b() {
        let input =
            "#######
             #E..EG#
             #.#G.E#
             #E.##E#
             #G..#.#
             #..E#.#
             #######";
        assert_eq!(part2(&as_input(input)), 31284);
    }

    #[test]
    fn test_part2_c() {
        let input =
            "#######
             #E.G#.#
             #.#G..#
             #G.#.G#
             #G..#.#
             #...E.#
             #######";
        assert_eq!(part2(&as_input(input)), 3478);
    }

    #[test]
    fn test_part2_d() {
        let input =
            "#######
             #.E...#
             #.#..G#
             #.###.#
             #E#G#G#
             #...#G#
             #######";
        assert_eq!(part2(&as_input(input)), 6474);
    }

    #[test]
    fn test_part2_e() {
        let input =
            "#########
             #G......#
             #.E.#...#
             #..##..G#
             #...##..#
             #...#...#
             #.G...G.#
             #.....G.#
             #########";
        assert_eq!(part2(&as_input(input)), 1140);
    }
}