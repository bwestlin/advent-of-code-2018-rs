extern crate utils;

use std::error::Error;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use utils::*;

#[derive(PartialEq, Eq, Clone)]
struct Landscape {
    grid: Vec<Vec<char>>,
    w: usize,
    h: usize
}

impl Landscape {
    fn parse(input: &Vec<String>) -> Landscape {
        let mut grid: Vec<Vec<char>> = vec![];
        for y in 0..input.len() {
            grid.push(input[y].chars().collect());
        }
        let w = grid[0].len();
        let h = grid.len();
        if !grid.iter().all(|gl| gl.len() == w) {
            panic!("Grid non uniform!");
        }
        Landscape { grid: grid, w: w, h: h }
    }

    fn adjacent(&self, x: usize, y: usize) -> Vec<char> {
        [(-1, -1), (0, -1), (1, -1), (-1, 0), (1, 0), (-1, 1), (0, 1), (1, 1)].iter()
            .map(|(dx, dy)| (x as i32 + dx, y as i32 + dy))
            .filter(|(ax, ay)| *ax >= 0 && *ay >= 0 && *ax < self.w as i32 && *ay < self.h as i32)
            .map(|(ax, ay)| self.grid[ay as usize][ax as usize])
            .collect()
    }

    fn next(&self) -> Landscape {
        let mut next = self.clone();
        for y in 0..self.h {
            for x in 0..self.w {
                let adj = self.adjacent(x, y);
                let cc = self.grid[y][x];
                let n_a_trees = adj.iter().filter(|&c| *c == '|').count();
                let n_a_lumbj = adj.iter().filter(|&c| *c == '#').count();

                let nc = match cc {
                    '.' if n_a_trees >= 3 => '|',
                    '|' if n_a_lumbj >= 3 => '#',
                    '#' if n_a_trees == 0 || n_a_lumbj == 0 => '.',
                    _ => cc
                };
                next.grid[y][x] = nc;
            }
        }
        next
    }

    fn count(&self, c: char) -> usize {
        self.grid.iter().flat_map(|l| l).filter(|&gc| *gc == c).count()
    }

    fn resource_value(&self) -> usize {
        self.count('|') * self.count('#')
    }

    #[cfg(feature = "print")]
    fn print(&self) {
        for y in 0..self.h {
            println!("{}", self.grid[y].iter().collect::<String>());
        }
    }
}

fn part1(input: &Vec<String>) -> usize {
    let landscape = Landscape::parse(input);

    #[cfg(feature = "print")] {
        println!("Initial state:");
        landscape.print();
    }

    let mut nl = landscape;
    for _i in 1..=10 {
        nl = nl.next();
        #[cfg(feature = "print")] {
            println!("After {} minutes:", _i);
            nl.print();
        }
    }

    nl.resource_value()
}

fn part2(input: &Vec<String>) -> usize {
    let landscape = Landscape::parse(input);

    #[cfg(feature = "print")] {
        println!("Initial state:");
        landscape.print();
    }

    let mut prev: Vec<(Landscape, usize, Vec<usize>)> = vec![];

    let mut nl = landscape;
    for _i in 0..=2000 {

        // Find matching previous iterations
        let mut m_prev_idx = vec![];
        for j in 0..prev.len() {
            if nl == prev[j].0 {
                m_prev_idx.push(j);
            }
        }
        if m_prev_idx.len() > 1 {
            break;
        }

        let rv = nl.resource_value();
        #[cfg(feature = "print")] {
            println!("After {} minutes:", _i);
            nl.print();
            println!("mins={:05}, rv={:05}, m_prev_idx={:?}", _i, rv, m_prev_idx);
        }

        let lnxt = nl.next();
        prev.push((nl, rv, m_prev_idx));
        nl = lnxt;
    }

    let rep_start = prev.iter()
        .enumerate()
        .skip_while(|(_, (_, _, prev))| prev.len() == 0)
        .next()
        .map(|(i, _)| i)
        .unwrap();
    let rep_interval = prev.len() - rep_start;

    let at_mins = 1_000_000_000;

    #[cfg(feature = "print")] {
        let (_, rv_start, _) = prev[rep_start];
        println!("rep_start={}, rep_interval={}, rv_start={}, at_mins={}", rep_start, rep_interval, rv_start, at_mins);
    }

    let (_, final_rv, _) = prev[rep_start + (at_mins - rep_start) % rep_interval];
    final_rv
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
    let f = File::open("src/day18/input")?;
    let f = BufReader::new(f);
    Ok(f.lines().map(|l| l.unwrap()).collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str =
       ".#.#...|#.
        .....#|##|
        .|..|...#.
        ..|#.....#
        #.#|||#|#|
        ...#.||...
        .|....|...
        ||...#|.#|
        |.||||..|.
        ...#.|..|.";

    fn as_input(s: &str) -> Vec<String> {
        s.split('\n').map(|s| s.trim().into()).collect()
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&as_input(INPUT)), 1147);
    }
}