extern crate utils;

use std::collections::HashMap;
use std::error::Error;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use utils::*;


fn part1(box_ids: Vec<String>) -> i32 {
    let (c2, c3) = box_ids.iter()
        .map(count_2or3_letters)
        .fold((0, 0), |(a2, a3), (c2, c3)| (a2 + c2, a3 + c3));
    c2 * c3
}

fn count_2or3_letters(s: &String) -> (i32, i32) {
    s.chars()
        .fold(HashMap::new(), |mut acc: HashMap<char, i32>, ch| {
            *acc.entry(ch).or_insert(0) += 1;
            acc
        })
        .values()
        .fold((0, 0), |(c2, c3), v| {
            (if c2 == 1 || *v == 2 { 1 } else { 0 }, if c3 == 1 || *v == 3 { 1 } else { 0 })
        })
}

fn part2(box_ids: &Vec<String>) -> String {
    let candidates: Vec<String> = box_ids.iter()
        .flat_map(|id| find_differs_by_one((*id).to_owned(), box_ids))
        .collect();

    candidates[0].chars().zip(candidates[1].chars())
        .filter(|(c1, c2)| c1 == c2)
        .map(|(ch, _)| ch)
        .collect()
}

fn find_differs_by_one(id: String, box_ids: &Vec<String>) -> Option<String> {
    box_ids.iter()
        .filter(|bid| {
            id.chars().zip(bid.chars()).filter(|(c1, c2)| c1 != c2).count() == 1
        })
        .map(|s| s.to_owned())
        .next()
}

fn main() -> Result<(), Box<Error>> {
    measure_exec(|| {
        let checksum = part1(input()?);
        println!("Part1 checksum: {}", checksum);
        Ok(())
    })?;
    measure_exec(|| {
        let common_letters = part2(&input()?);
        println!("Part2 common letters: {}", common_letters);
        Ok(())
    })?;
    Ok(())
}

fn input() -> io::Result<Vec<String>> {
    let f = File::open("src/day02/input")?;
    let f = BufReader::new(f);
    Ok(f.lines().map(|l| l.unwrap()).collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn to_input(s: &str) -> Vec<String> {
        s.split(',').map(|s| s.trim().to_string()).collect()
    }

    #[test]
    fn test_count_2or3_letters() {
        assert_eq!(count_2or3_letters(&"abcdef".to_string()), (0, 0));
        assert_eq!(count_2or3_letters(&"bababc".to_string()), (1, 1));
        assert_eq!(count_2or3_letters(&"abbcde".to_string()), (1, 0));
        assert_eq!(count_2or3_letters(&"abcccd".to_string()), (0, 1));
        assert_eq!(count_2or3_letters(&"aabcdd".to_string()), (1, 0));
        assert_eq!(count_2or3_letters(&"abcdee".to_string()), (1, 0));
        assert_eq!(count_2or3_letters(&"ababab".to_string()), (0, 1));
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(to_input("abcdef, bababc, abbcde, abcccd, aabcdd, abcdee, ababab")), 12);
    }

    #[test]
    fn test_find_differs_by_one() {
        assert_eq!(find_differs_by_one("fghij".to_string(), &to_input("abcde, fghij, klmno, pqrst, fguij, axcye, wvxyz")), Some("fguij".to_string()));
        assert_eq!(find_differs_by_one("fguij".to_string(), &to_input("abcde, fghij, klmno, pqrst, fguij, axcye, wvxyz")), Some("fghij".to_string()));
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&to_input("abcde, fghij, klmno, pqrst, fguij, axcye, wvxyz")), "fgij".to_string());
    }
}