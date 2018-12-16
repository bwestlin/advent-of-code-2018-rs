extern crate utils;

use std::error::Error;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use utils::*;

#[cfg(feature = "print")]
fn print_scores(recipe_scores: &Vec<u8>, elves_idx: &[usize; 2]) {
    for si in 0..recipe_scores.len() {
        if si == elves_idx[0] {
            print!("({})", recipe_scores[si]);
        } else if si == elves_idx[1] {
            print!("[{}]", recipe_scores[si]);
        } else {
            print!(" {} ", recipe_scores[si]);
        }
    }
    println!();
}

fn part1(input: String) -> Result<String, Box<Error>> {
    let mut recipe_scores = vec![3u8,7u8];
    let mut elves_idx = [0, 1];
    let n_recipes = input.parse::<usize>()?;

    #[cfg(feature = "print")] {
        println!("part1({})", input);
        print_scores(&recipe_scores, &elves_idx);
    }

    loop {
        let sum = recipe_scores[elves_idx[0]] + recipe_scores[elves_idx[1]];
        if sum > 9 {
            recipe_scores.push(sum / 10);
        }
        recipe_scores.push(sum % 10);

        for j in 0..2 {
            elves_idx[j] = (elves_idx[j] + recipe_scores[elves_idx[j]] as usize + 1) % recipe_scores.len();
        }

        #[cfg(feature = "print")] print_scores(&recipe_scores, &elves_idx);

        if recipe_scores.len() > 10 && recipe_scores.len() - 10 >= n_recipes {
            break;
        }
    }

    Ok(recipe_scores[n_recipes..(n_recipes + 10)].iter().map(|&s| ('0' as u8 + s) as char).collect())
}

fn part2(input: String) -> usize {
    let mut recipe_scores = vec![3u8,7u8];
    let mut elves_idx = [0, 1];

    let first_recipes: Vec<_> = input.chars().map(|c| 9 - ('9' as u8 - c as u8) as u8).collect();
    let mut s_idx = 0;

    #[cfg(feature = "print")] {
        println!("part2({})", input);
        print_scores(&recipe_scores, &elves_idx);
    }

    loop {
        let sum = recipe_scores[elves_idx[0]] + recipe_scores[elves_idx[1]];
        if sum > 9 {
            recipe_scores.push(sum / 10);
        }
        recipe_scores.push(sum % 10);

        for j in 0..2 {
            elves_idx[j] = (elves_idx[j] + recipe_scores[elves_idx[j]] as usize + 1) % recipe_scores.len();
        }

        #[cfg(feature = "print")] print_scores(&recipe_scores, &elves_idx);

        if recipe_scores.len() > first_recipes.len() && recipe_scores.len() - s_idx >= first_recipes.len() {
            let num = (recipe_scores.len() - s_idx) - first_recipes.len() + 1;

            for _ in 0..num {
                let scrs = &recipe_scores[(s_idx)..(s_idx + first_recipes.len())];

                if scrs == &first_recipes[..] {
                    return s_idx;
                }
                s_idx += 1;
            }
        }
    }
}

fn main() -> Result<(), Box<Error>> {
    measure_exec(|| {
        let result = part1(input()?)?;
        println!("Part1 result: {}", result);
        Ok(())
    })?;
    measure_exec(|| {
        let result = part2(input()?);
        println!("Part2 result: {}", result);
        Ok(())
    })?;
    Ok(())
}

fn input() -> io::Result<String> {
    let f = File::open("src/day14/input")?;
    let f = BufReader::new(f);
    Ok(f.lines().map(|l| l.unwrap()).next().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() -> Result<(), Box<Error>> {
        assert_eq!(part1("5".into())?, "0124515891".to_string());
        assert_eq!(part1("9".into())?, "5158916779".to_string());
        assert_eq!(part1("18".into())?, "9251071085".to_string());
        assert_eq!(part1("2018".into())?, "5941429882".to_string());
        Ok(())
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("51589".into()), 9);
        assert_eq!(part2("01245".into()), 5);
        assert_eq!(part2("92510".into()), 18);
        assert_eq!(part2("59414".into()), 2018);
    }
}