extern crate utils;

use std::error::Error;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use utils::*;

fn find_pos_highest_power(ser_no: usize, smin: usize, smax: usize) -> (usize, usize, usize, i32) {
    const N: usize = 300;
    let mut grid = [[0i32; N]; N];

    for y in 0..N {
        for x in 0..N {
            let r_id = (x + 1) + 10;
            let p_lev_s = r_id * (y + 1);
            let h_dig = (((p_lev_s + ser_no) * r_id) / 100) % 10;
            let p_lev = h_dig as i32 - 5;
            grid[y][x] = p_lev;
        }
    }

    let mut ltot_p = 0;
    let mut lpos = (0, 0, 0, 0);

    // Naive first implementation
    // for s in smin..=smax {
    //     for y in 0..(N - (s - 1)) {
    //         for x in 0..(N - (s - 1)) {
    //             let mut tot_p = 0;
    //             for sy in y..(y + s) {
    //                 for sx in x..(x + s) {
    //                     tot_p += grid[sy][sx];
    //                 }
    //             }
    //             if tot_p > ltot_p {
    //                 ltot_p = tot_p;
    //                 lpos = (x + 1, y + 1, s, tot_p);
    //             }
    //         }
    //     }
    // }

    // Optimized implementation, ~65x faster than naive
    for y in 0..(N - smin) {
        for x in 0..(N - smin) {
            let mut tot_p = 0;

            for sy in 0..(smin - 1) {
                for sx in 0..(smin - 1) {
                    tot_p += grid[y + sy][x + sx];
                }
            }

            let smx = std::cmp::min(std::cmp::min(N - y, N - x), smax);
            if smx >= smin {
                for s in smin..=smx {
                    for sx in x..(x + s) {
                        tot_p += grid[y + s - 1][sx];
                    }
                    for sy in y..(y + s - 1) {
                        tot_p += grid[sy][x + s - 1];
                    }
                    if tot_p > ltot_p {
                        ltot_p = tot_p;
                        lpos = (x + 1, y + 1, s, tot_p);
                    }
                }
            }
        }
    }

    lpos
}

fn part1(ser_no: usize) -> (usize, usize) {
    let (x, y, _, _) = find_pos_highest_power(ser_no, 3, 3);
    (x, y)
}

fn part2(ser_no: usize) -> (usize, usize, usize) {
    let (x, y, s, _) = find_pos_highest_power(ser_no, 1, 300);
    (x, y, s)
}

fn main() -> Result<(), Box<Error>> {
    measure_exec(|| {
        let result = part1(input()?);
        println!("Part1 result: {:?}", result);
        Ok(())
    })?;
    measure_exec(|| {
        let result = part2(input()?);
        println!("Part2 result: {:?}", result);
        Ok(())
    })?;
    Ok(())
}

fn input() -> io::Result<usize> {
    let f = File::open("src/day11/input")?;
    let f = BufReader::new(f);
    Ok(f.lines().map(|l| l.unwrap().parse::<usize>().unwrap()).next().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn as_input(s: &str) -> usize {
        s.split('\n').map(|s| s.trim().parse::<usize>().unwrap()).next().unwrap()
    }

    #[test]
    fn test_find_pos_highest_power() {
        assert_eq!(find_pos_highest_power(18, 3, 3), (33, 45, 3, 29));
        assert_eq!(find_pos_highest_power(42, 3, 3), (21, 61, 3, 30));
        assert_eq!(find_pos_highest_power(18, 1, 300), (90, 269, 16, 113));
        assert_eq!(find_pos_highest_power(42, 1, 300), (232, 251, 12, 119));
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(as_input("18")), (33, 45));
        assert_eq!(part1(as_input("42")), (21, 61));
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(as_input("18")), (90, 269, 16));
        assert_eq!(part2(as_input("42")), (232, 251, 12));
    }
}