extern crate utils;

use std::error::Error;
use std::collections::BTreeSet;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use utils::*;

struct Tracks {
    grid: Vec<Vec<char>>
}

impl Tracks {
    fn parse_input(input: &Vec<String>) -> (Tracks, Vec<Cart>) {
        let mut grid: Vec<Vec<char>> = vec![];
        let mut carts: Vec<Cart> = vec![];

        for y in 0..input.len() {
            let l: Vec<_> = input[y].chars().collect();
            let mut chars = vec![];

            for x in 0..l.len() {
                let c = l[x];
                let c = match Dir::try_parse(c) {
                    Some(dir) => {
                        carts.push(Cart { pos: (x, y), dir: dir.to_owned(), turn: Turn::Left });
                        match dir {
                            Dir::Right => '-',
                            Dir::Down  => '|',
                            Dir::Left  => '-',
                            Dir::Up    => '|'
                        }
                    },
                    None => c
                };

                chars.push(c);
            }
            grid.push(chars);
        }

        (Tracks { grid: grid }, carts)
    }

    fn at(&self, x: usize, y: usize) -> char {
        self.grid[y][x]
    }

    fn print(&self, carts: &Vec<Cart>) {
        for y in 0..self.grid.len() {
            for x in 0..(self.grid[y].len()) {
                let c = self.grid[y][x];
                let c = carts.iter()
                    .filter(|Cart { pos: (cx, cy), dir: _, turn: _ }| *cx == x && *cy == y)
                    .map(Cart::as_char)
                    .next()
                    .unwrap_or(c);
                print!("{}", c);
            }
            println!();
        }

    }
}

#[derive(Debug, Clone, Copy)]
enum Dir {
    Right, Down, Left, Up
}

impl Dir {
    fn try_parse(c: char) -> Option<Dir> {
        match c {
            '>' => Some(Dir::Right),
            'v' => Some(Dir::Down),
            '<' => Some(Dir::Left),
            '^' => Some(Dir::Up),
            _ => None
        }
    }

    fn turn(self, turn: Turn) -> Dir {
        match (self, turn) {
            (Dir::Right, Turn::Left)  => Dir::Up,
            (Dir::Down,  Turn::Left)  => Dir::Right,
            (Dir::Left,  Turn::Left)  => Dir::Down,
            (Dir::Up,    Turn::Left)  => Dir::Left,
            (Dir::Right, Turn::Right) => Dir::Down,
            (Dir::Down,  Turn::Right) => Dir::Left,
            (Dir::Left,  Turn::Right) => Dir::Up,
            (Dir::Up,    Turn::Right) => Dir::Right,
            _ => self
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Turn {
    Left, Straight, Right
}


#[derive(Debug, Clone)]
struct Cart {
    pos: (usize, usize),
    dir: Dir,
    turn: Turn
}

impl Cart {
    fn as_char(&self) -> char {
        match self.dir {
            Dir::Right => '>',
            Dir::Down  => 'v',
            Dir::Left  => '<',
            Dir::Up    => '^'
        }
    }

    fn move_one(&mut self, tracks: &Tracks) {
        let (cx, cy) = self.pos;
        let (nx, ny) = match self.dir {
            Dir::Right => (cx + 1, cy),
            Dir::Down  => (cx, cy + 1),
            Dir::Left  => (cx - 1, cy),
            Dir::Up    => (cx, cy - 1)
        };

        match (tracks.at(nx, ny), self.dir) {
            ('+', _) => {
                self.dir = self.dir.turn(self.turn);
                self.turn = match self.turn {
                    Turn::Left     => Turn::Straight,
                    Turn::Straight => Turn::Right,
                    Turn::Right    => Turn::Left
                };
            },
            ('/',  Dir::Up)    => self.dir = Dir::Right,
            ('/',  Dir::Left)  => self.dir = Dir::Down,
            ('/',  Dir::Down)  => self.dir = Dir::Left,
            ('/',  Dir::Right) => self.dir = Dir::Up,
            ('\\', Dir::Up)    => self.dir = Dir::Left,
            ('\\', Dir::Left)  => self.dir = Dir::Up,
            ('\\', Dir::Down)  => self.dir = Dir::Right,
            ('\\', Dir::Right) => self.dir = Dir::Down,
            _ => {}
        }

        self.pos = (nx, ny);
    }
}

fn first_chrash_pos(tracks: &Tracks, carts: &Vec<Cart>) -> (usize, usize) {
    (0..)
        .scan((carts.to_vec(), false), |(ref mut carts, ref mut collided), i| {
            // println!("Iteration {}:", i);
            // tracks.print(&carts);
            if *collided {
                return None
            }

            (*carts).sort_unstable_by(|a, b| a.pos.1.cmp(&b.pos.1).then(a.pos.0.cmp(&b.pos.0)));

            for i in 0..carts.len() {
                carts[i].move_one(tracks);
                for j in 0..carts.len() {
                    if i != j && carts[i].pos == carts[j].pos {
                        *collided = true;
                        return Some(Some(carts[i].pos))
                    }
                }
            }
            Some(None)
        })
        .last()
        .unwrap()
        .unwrap()
}

fn part1(input: &Vec<String>) -> (usize, usize) {
    let (tracks, carts) = Tracks::parse_input(input);
    first_chrash_pos(&tracks, &carts)
}

fn last_remaining_pos(tracks: &Tracks, carts: &Vec<Cart>) -> (usize, usize) {
    (0..)
        .scan(carts.to_vec(), |carts, i| {
            // println!("Iteration {}:", i);
            // tracks.print(&carts);
            if carts.len() == 1 {
                return None
            }

            (*carts).sort_unstable_by(|a, b| a.pos.1.cmp(&b.pos.1).then(a.pos.0.cmp(&b.pos.0)));

            let mut collided_idx = BTreeSet::new();
            for i in 0..carts.len() {
                if !collided_idx.contains(&i) {
                    carts[i].move_one(tracks);
                    for j in 0..carts.len() {
                        if i != j && !collided_idx.contains(&j) && carts[i].pos == carts[j].pos {
                            collided_idx.insert(i);
                            collided_idx.insert(j);
                        }
                    }
                }
            }
            for (i, idx) in collided_idx.iter().enumerate() {
                (*carts).remove(idx - i);
            }

            if carts.len() == 1 {
                Some(Some(carts[0].pos))
            } else {
                Some(None)
            }
        })
        .last()
        .unwrap()
        .unwrap()
}

fn part2(input: &Vec<String>) -> (usize, usize) {
    let (tracks, carts) = Tracks::parse_input(input);
    last_remaining_pos(&tracks, &carts)
}

fn main() -> Result<(), Box<Error>> {
    measure_exec(|| {
        let result = part1(&input()?);
        println!("Part1 result: {:?}", result);
        Ok(())
    })?;
    measure_exec(|| {
        let result = part2(&input()?);
        println!("Part2 result: {:?}", result);
        Ok(())
    })?;
    Ok(())
}

fn input() -> io::Result<Vec<String>> {
    let f = File::open("src/day13/input")?;
    let f = BufReader::new(f);
    Ok(f.lines().map(|l| l.unwrap()).collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn as_input(s: &str) -> Vec<String> {
        s.split('\n').map(|s| s.trim().to_string().chars().skip(1).take_while(|&c| c != '.').collect()).collect()
    }

    const INPUT1: &'static str =
      r"./->-\        .
        .|   |  /----\.
        .| /-+--+-\  |.
        .| | |  | v  |.
        .\-+-/  \-+--/.
        .  \------/   .";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&as_input(INPUT1)), (7, 3));
    }

    const INPUT2: &'static str =
      r"./>-<\  .
        .|   |  .
        .| /<+-\.
        .| | | v.
        .\>+</ |.
        .  |   ^.
        .  \<->/.";

    #[test]
    fn test_part2() {
        assert_eq!(part2(&as_input(INPUT2)), (6, 4));
    }
}