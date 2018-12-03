use std::collections::HashSet;
use std::error::Error;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;


fn part1(freq_changes: Vec<i32>) -> i32 {
    freq_changes.iter().sum()
}

fn part2(freq_changes: Vec<i32>) -> Option<i32> {
    let mut freq = 0;
    let mut reached_freqs = HashSet::new();
    reached_freqs.insert(freq);

    for _ in 0..freq_changes.len() {
        for change in freq_changes.iter() {
            freq += change;

            if !reached_freqs.insert(freq) {
                return Some(freq);
            }
        }
    }
    None
}

fn main() -> Result<(), Box<Error>> {
    let res_freq = part1(input()?);
    println!("Part1 result frequency: {:?}", res_freq);

    let freq_repeat = part2(input()?);
    println!("Part2 first repeated frequency: {:?}", freq_repeat.unwrap());

    Ok(())
}

fn input() -> io::Result<Vec<i32>> {
    let f = File::open("src/day01/input")?;
    let f = BufReader::new(f);
    Ok(f.lines().map(|l| l.unwrap().parse::<i32>().unwrap()).collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn to_input(s: &str) -> Vec<i32> {
        s.split(',').map(|s| s.trim().to_string().parse::<i32>().unwrap()).collect()
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(to_input("+1, -2, +3, +1")), 3);
        assert_eq!(part1(to_input("+1, +1, +1")), 3);
        assert_eq!(part1(to_input("+1, +1, -2")), 0);
        assert_eq!(part1(to_input("-1, -2, -3")), -6);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(to_input("+1, -2, +3, +1")), Some(2));
        assert_eq!(part2(to_input("+1, -1")), Some(0));
        assert_eq!(part2(to_input("+3, +3, +4, -2, -4")), Some(10));
        assert_eq!(part2(to_input("-6, +3, +8, +5, -6")), Some(5));
        assert_eq!(part2(to_input("+7, +7, -2, -7, -4")), Some(14));
    }
}