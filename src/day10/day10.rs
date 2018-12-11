extern crate regex;
#[macro_use] extern crate lazy_static;
extern crate utils;

use std::error::Error;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use regex::Regex;
use utils::*;

#[derive(Clone)]
struct Vec2 {
    x: i32,
    y: i32
}

#[derive(Clone)]
struct Point {
    pos: Vec2,
    vel: Vec2
}

fn parse_points(input: &Vec<String>) -> Vec<Point> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^.+<\s*(-?\d+),\s*(-?\d+)>.+<\s*(-?\d+),\s*(-?\d+)>$").unwrap();
    }
    input.iter()
        .map(|i| {
            let caps = RE.captures(i).unwrap();
            let get = |idx| caps.get(idx).unwrap().as_str().parse::<i32>().unwrap();
            Point {
                pos: Vec2 { x: get(1), y: get(2) },
                vel: Vec2 { x: get(3), y: get(4) }
            }
        })
        .collect()
}

fn tot_height(points: &Vec<Point>) -> i32 {
    let (miny, maxy) = points.iter()
        .fold((points[0].pos.y, points[0].pos.y), |(miny, maxy), p| (std::cmp::min(miny, p.pos.y), std::cmp::max(maxy, p.pos.y)));
    maxy - miny
}

fn move_points(points: &mut Vec<Point>, dir: i32) {
    for p in points {
        p.pos.x += p.vel.x * dir;
        p.pos.y += p.vel.y * dir;
    }
}

fn gen_message(points: &Vec<Point>) -> String {
    let min_x = points.iter().map(|p| p.pos.x).min().unwrap();
    let max_x = points.iter().map(|p| p.pos.x).max().unwrap();
    let min_y = points.iter().map(|p| p.pos.y).min().unwrap();
    let max_y = points.iter().map(|p| p.pos.y).max().unwrap();

    let mut m = vec![vec!['.'; (max_x - min_x + 1)  as usize]; (max_y - min_y + 1) as usize];
    for p in points {
        m[(p.pos.y - min_y) as usize][(p.pos.x - min_x) as usize] = '#';
    }
    m.iter().map(|l| { let mut s = l.into_iter().collect::<String>(); s.push('\n'); s }).collect()
}

// An imperative approach to finding the message
fn find_message(input: &Vec<String>) -> (String, i32) {
    let mut points = parse_points(input);
    let points = points.as_mut();
    let mut ltoth = tot_height(&points);
    let mut nsec = 0;

    loop {
        move_points(points, 1);
        let toth = tot_height(&points);
        if toth > ltoth {
            break;
        }
        ltoth = toth;
        nsec += 1;
    }
    move_points(points, -1);

    (gen_message(points), nsec)
}

// A functional approach to finding the message
fn find_message_func(input: &Vec<String>) -> (String, i32) {
    let points = parse_points(input);
    let th = tot_height(&points);

    (1..)
        .scan((points, th), |(ref mut lpts, ref mut lth), i| {
            let pts: Vec<_> = lpts.iter()
                .map(|p| Point { pos: Vec2 { x: p.pos.x + p.vel.x, y: p.pos.y + p.vel.y }, vel: p.vel.to_owned() })
                .collect();

            let th = tot_height(&pts);
            if th > *lth {
                None
            } else {
                *lpts = pts.to_owned();
                *lth = th;
                Some((pts, i))
            }
        })
        .last()
        .map(|(rpoints, tsecs)| (gen_message(&rpoints), tsecs))
        .unwrap()
}

fn part1(input: &Vec<String>) -> String {
    let (message, _) = find_message(input);
    message
}

fn part1_func(input: &Vec<String>) -> String {
    let (message, _) = find_message_func(input);
    message
}

fn part2(input: &Vec<String>) -> i32 {
    let (_, count) = find_message(input);
    count
}

fn part2_func(input: &Vec<String>) -> i32 {
    let (_, count) = find_message_func(input);
    count
}

fn main() -> Result<(), Box<Error>> {
    measure_exec(|| {
        let result = part1(&input()?);
        println!("Part1 result:\n{}", result);
        Ok(())
    })?;
    measure_exec(|| {
        let result = part1_func(&input()?);
        println!("Part1 func result:\n{}", result);
        Ok(())
    })?;
    measure_exec(|| {
        let result = part2(&input()?);
        println!("Part2 result: {}", result);
        Ok(())
    })?;
    measure_exec(|| {
        let result = part2_func(&input()?);
        println!("Part2 func result: {}", result);
        Ok(())
    })?;
    Ok(())
}

fn input() -> io::Result<Vec<String>> {
    let f = File::open("src/day10/input")?;
    let f = BufReader::new(f);
    Ok(f.lines().map(|l| l.unwrap()).collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str =
       "position=< 9,  1> velocity=< 0,  2>
        position=< 7,  0> velocity=<-1,  0>
        position=< 3, -2> velocity=<-1,  1>
        position=< 6, 10> velocity=<-2, -1>
        position=< 2, -4> velocity=< 2,  2>
        position=<-6, 10> velocity=< 2, -2>
        position=< 1,  8> velocity=< 1, -1>
        position=< 1,  7> velocity=< 1,  0>
        position=<-3, 11> velocity=< 1, -2>
        position=< 7,  6> velocity=<-1, -1>
        position=<-2,  3> velocity=< 1,  0>
        position=<-4,  3> velocity=< 2,  0>
        position=<10, -3> velocity=<-1,  1>
        position=< 5, 11> velocity=< 1, -2>
        position=< 4,  7> velocity=< 0, -1>
        position=< 8, -2> velocity=< 0,  1>
        position=<15,  0> velocity=<-2,  0>
        position=< 1,  6> velocity=< 1,  0>
        position=< 8,  9> velocity=< 0, -1>
        position=< 3,  3> velocity=<-1,  1>
        position=< 0,  5> velocity=< 0, -1>
        position=<-2,  2> velocity=< 2,  0>
        position=< 5, -2> velocity=< 1,  2>
        position=< 1,  4> velocity=< 2,  1>
        position=<-2,  7> velocity=< 2, -2>
        position=< 3,  6> velocity=<-1, -1>
        position=< 5,  0> velocity=< 1,  0>
        position=<-6,  0> velocity=< 2,  0>
        position=< 5,  9> velocity=< 1, -2>
        position=<14,  7> velocity=<-2,  0>
        position=<-3,  6> velocity=< 2, -1>";

    fn as_input(s: &str) -> Vec<String> {
        s.split('\n').map(|s| s.trim().into()).collect()
    }

    fn as_display(s: &str) -> String {
        s.split('\n').map(|s| { let mut st: String = s.trim().into(); st.push('\n'); st }).collect()
    }

    #[test]
    fn test_part1() {
        let expected = as_display(
            "#...#..###
             #...#...#.
             #...#...#.
             #####...#.
             #...#...#.
             #...#...#.
             #...#...#.
             #...#..###"
        );

        assert_eq!(part1(&as_input(INPUT)), expected);
    }

    #[test]
    fn test_part1_func() {
        let expected = as_display(
            "#...#..###
             #...#...#.
             #...#...#.
             #####...#.
             #...#...#.
             #...#...#.
             #...#...#.
             #...#..###"
        );

        assert_eq!(part1_func(&as_input(INPUT)), expected);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&as_input(INPUT)), 3);
    }

    #[test]
    fn test_part2_func() {
        assert_eq!(part2_func(&as_input(INPUT)), 3);
    }
}