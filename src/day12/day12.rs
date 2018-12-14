extern crate utils;

use std::error::Error;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use utils::*;

fn parse_input(input: &Vec<String>) -> (Vec<bool>, Vec<Vec<bool>>) {
    let init_state: Vec<_> = (input[0])["initial state: ".len()..]
        .chars()
        .map(|c| c == '#')
        .collect();

    let growth_patterns: Vec<_> = input.iter()
        .skip(2)
        .filter(|l| l.ends_with(" => #"))
        .map(|l| {
            l.chars().take(5).map(|c| c == '#').collect()
        })
        .collect();

    (init_state, growth_patterns)
}

// fn print_state(state: &Vec<bool>, s_idx: usize) {
//     for &p in state.iter().skip(20) {
//         print!("{}", if p { "#" } else { "." });
//     }
//     println!();
// }

fn run_gen(state: &Vec<bool>, growth_patters: &Vec<Vec<bool>>) -> Vec<bool> {
    let mut next_state = vec![false; state.len() + 1];

    for i in 2..(state.len() - 2) {
        for j in 0..growth_patters.len() {
            let gp = &growth_patters[j];
            if gp[0] == state[i - 2] &&
               gp[1] == state[i - 1] &&
               gp[2] == state[i - 0] &&
               gp[3] == state[i + 1] &&
               gp[4] == state[i + 2] {
                next_state[i] = true;
                break;
            }
        }
    }

    next_state
}

fn pad_state(state: &Vec<bool>, pad: usize) -> Vec<bool> {
    let p_iter = || [false].iter().cycle().take(pad);
    p_iter().chain(state.iter()).chain(p_iter()).map(|b| *b).collect()
}

fn part1(input: &Vec<String>) -> i32 {
    let (init_state, growth_patters) = parse_input(input);

    let s_idx = 5;
    let mut state: Vec<bool> = pad_state(&init_state, s_idx);
    //print_state(&state, s_idx);
    for _ in 0..20 {
        state = run_gen(&state, &growth_patters);
        //print_state(&state, s_idx);
    }

    let plant_idxs: Vec<i32> = state.iter()
        .enumerate()
        .filter(|(_, &p)| p)
        .map(|(i, _)| i as i32 - s_idx as i32)
        .collect();

    plant_idxs.iter().sum()
}

fn part2(input: &Vec<String>) -> usize {
    let (init_state, growth_patters) = parse_input(input);

    let s_idx = 5;
    let mut state: Vec<bool> = pad_state(&init_state, s_idx);
    //print_state(&state, s_idx);
    let mut s_gen = 0;

    // We assume here that a stable pattern can be reached within 200 iterations
    // This is based purely on obeservation of the input data
    for g in 0..200 {
        let next_state = run_gen(&state, &growth_patters);
        //print_state(&next_state, s_idx);

        let stable = state.iter().zip(next_state.iter().skip(1)).all(|(a, b)| a == b);
        if stable {
            s_gen = g;
            break;
        }
        state = next_state;
    }

    let i_add = 50_000_000_000 - s_gen;

    let plant_idxs: Vec<usize> = state.iter()
        .enumerate()
        .filter(|(_, &p)| p)
        .map(|(i, _)| i - s_idx)
        .map(|i| i + i_add)
        .collect();

    plant_idxs.iter().sum()
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
    let f = File::open("src/day12/input.king")?;
    let f = BufReader::new(f);
    Ok(f.lines().map(|l| l.unwrap()).collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str =
       "initial state: #..#.#..##......###...###

        ...## => #
        ..#.. => #
        .#... => #
        .#.#. => #
        .#.## => #
        .##.. => #
        .#### => #
        #.#.# => #
        #.### => #
        ##.#. => #
        ##.## => #
        ###.. => #
        ###.# => #
        ####. => #";

    fn as_input(s: &str) -> Vec<String> {
        s.split('\n').map(|s| s.trim().into()).collect()
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&as_input(INPUT)), 325);
    }
}