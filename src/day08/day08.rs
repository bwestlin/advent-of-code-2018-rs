extern crate utils;

use std::error::Error;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use utils::*;

fn extract_metadata(xs: &[i32]) -> (usize, Vec<i32>) {
    let n_child = xs[0];
    let n_metad = xs[1] as usize;
    let mut c_len = 0;
    let mut metad: Vec<i32> = vec![];

    for _ in 0..n_child {
        let (cl, ref mut cmetad) = extract_metadata(&xs[(2 + c_len)..]);
        metad.append(cmetad);
        c_len += cl;
    }

    for mi in 0..n_metad {
        metad.push(xs[2 + c_len + mi]);
    }

    (2 + c_len + n_metad, metad)
}

fn part1(input: &Vec<i32>) -> i32 {
    let (_, metadata) = extract_metadata(input);
    metadata.iter().sum()
}

fn extract_value(xs: &[i32]) -> (usize, i32) {
    let n_child = xs[0] as usize;
    let n_metad = xs[1] as usize;
    let mut c_len = 0;
    let mut c_value = vec![0i32; n_child];

    if n_child > 0 {
        for ci in 0..n_child {
            let (cl, cv) = extract_value(&xs[(2 + c_len)..]);
            c_value[ci] = cv;
            c_len += cl;
        }

        let ms = 2 + c_len;
        let value = xs[ms..(ms + n_metad)].iter()
            .map(|ci| ci - 1)
            .filter(|&ci| ci >= 0 && ci < c_value.len() as i32)
            .map(|ci| {
                c_value[ci as usize]
            })
        .sum();

        (2 + c_len + n_metad, value)
    } else {
        (2 + c_len + n_metad, xs[2..(2 + n_metad)].iter().sum())
    }
}

fn part2(input: &Vec<i32>) -> i32 {
    let (_, value) = extract_value(input);
    value
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