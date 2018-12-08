extern crate utils;

use std::error::Error;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use utils::*;

pub struct Node {
    pub children: Vec<Node>,
    pub metadata: Vec<i32>
}

fn parse_tree(xs: &[i32]) -> (usize, Node) {
    let (c_len, children) = (0..xs[0]).into_iter()
        .fold((0, vec![]), |(cl, mut ch): (usize, Vec<Node>), _| {
            let (c_len, node) = parse_tree(&xs[(2 + cl)..]);
            ch.push(node);
            (cl + c_len, ch)
        });
    let m_cnt = xs[1] as usize;
    let m_idx = 2 + c_len;
    (2 + c_len + m_cnt, Node { children: children, metadata: xs[m_idx..(m_idx + m_cnt)].to_vec() })
}

fn sum_metadata(node: &Node) -> i32 {
    (*node).metadata.iter().sum::<i32>() + (*node).children.iter().map(sum_metadata).sum::<i32>()
}

fn part1(input: &Vec<i32>) -> i32 {
    let (_, root) = parse_tree(input);
    sum_metadata(&root)
}

fn sum_value(node: &Node) -> i32 {
    if (*node).children.len() == 0 {
        (*node).metadata.iter().sum()
    } else {
        (*node).metadata.iter()
            .map(|ci| (ci - 1) as usize)
            .filter(|&ci| ci < (*node).children.len())
            .map(|ci| {
                sum_value(&(*node).children[ci])
            })
            .sum()
    }
}

fn part2(input: &Vec<i32>) -> i32 {
    let (_, root) = parse_tree(input);
    sum_value(&root)
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

fn input() -> io::Result<Vec<i32>> {
    let f = File::open("src/day08/input")?;
    let f = BufReader::new(f);
    Ok(f.lines().next().unwrap().unwrap().split(' ').map(|l| l.parse().unwrap()).collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str =
       "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2";

    fn as_input(s: &str) -> Vec<i32> {
        s.split('\n').next().unwrap().split(' ').map(|s| s.parse().unwrap()).collect()
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&as_input(INPUT)), 138);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&as_input(INPUT)), 66);
    }
}