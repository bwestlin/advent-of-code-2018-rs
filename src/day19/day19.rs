extern crate regex;
#[macro_use] extern crate lazy_static;
extern crate utils;

use std::fmt;
use std::error::Error;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use regex::Regex;
use utils::*;

fn parse_input(input: &Vec<String>) -> (usize, Vec<Instruction>) {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^(\D+)\D+(\d+)\D+(\d+)\D+(\d+)\D*$").unwrap();
    }
    let get_vals = |s: &String| {
        let caps = RE.captures(s).unwrap();
        let get_u = |idx| caps.get(idx).unwrap().as_str().parse::<u32>().unwrap();
        let get_s = |idx| caps.get(idx).unwrap().as_str();
        Instruction { opc: OpCode::parse(get_s(1)), inp_a: get_u(2), inp_b: get_u(3), out_c: get_u(4) }
    };

    let mut ipb = 0;
    let mut instructions: Vec<Instruction> = vec![];

    let mut idx = 0;
    while idx < input.len() {
        let l = &input[idx];
        if l.starts_with("#ip ") {
            ipb = l.rsplit(' ').next().unwrap().parse::<usize>().unwrap();
        } else if l.trim().len() > 0 {
            instructions.push(get_vals(l));
        }
        idx += 1;
    }
    (ipb, instructions)
}

#[derive(Clone, Copy, Debug)]
enum OpCode {
    Addr, Addi, Mulr, Muli, Banr, Bani, Borr, Bori, Setr, Seti, Gtir, Gtri, Gtrr, Eqir, Eqri, Eqrr
}

impl OpCode {
    fn parse(s: &str) -> OpCode {
        use OpCode::*;
        match s {
            "addr" => Addr,
            "addi" => Addi,
            "mulr" => Mulr,
            "muli" => Muli,
            "banr" => Banr,
            "bani" => Bani,
            "borr" => Borr,
            "bori" => Bori,
            "setr" => Setr,
            "seti" => Seti,
            "gtir" => Gtir,
            "gtri" => Gtri,
            "gtrr" => Gtrr,
            "eqir" => Eqir,
            "eqri" => Eqri,
            "eqrr" => Eqrr,
            _ => unreachable!()
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Instruction {
    opc: OpCode,
    inp_a: u32,
    inp_b: u32,
    out_c: u32
}

#[cfg(feature = "print")]
impl Instruction {
    fn explain(&self, ipb: usize) -> String {
        use OpCode::*;
        let mut r = vec!["a", "b", "c", "d", "e", "f"].iter().map(|s| s.to_string()).collect::<Vec<_>>();
        r[ipb] = format!("{}(ip)", r[ipb]);
        let a = self.inp_a;
        let ai = a as usize;
        let b = self.inp_b;
        let bi = b as usize;
        let c = self.out_c;
        let ci = c as usize;
        match self.opc {
            Addr => format!("{} = {} + {}", r[ci], r[ai], r[bi]),
            Addi => format!("{} = {} + {}", r[ci], r[ai], b),
            Mulr => format!("{} = {} * {}", r[ci], r[ai], r[bi]),
            Muli => format!("{} = {} * {}", r[ci], r[ai], b),
            Banr => format!("{} = {} & {}", r[ci], r[ai], r[bi]),
            Bani => format!("{} = {} & {}", r[ci], r[ai], b),
            Borr => format!("{} = {} | {}", r[ci], r[ai], r[bi]),
            Bori => format!("{} = {} | {}", r[ci], r[ai], b),
            Setr => format!("{} = {}", r[ci], r[ai]),
            Seti => format!("{} = {}", r[ci], a),
            Gtir => format!("{} = if {} > {} {{ 1 }} else {{ 0 }}", r[ci], a, r[bi]),
            Gtri => format!("{} = if {} > {} {{ 1 }} else {{ 0 }}", r[ci], r[ai], b),
            Gtrr => format!("{} = if {} > {} {{ 1 }} else {{ 0 }}", r[ci], r[ai], r[bi]),
            Eqir => format!("{} = if {} == {} {{ 1 }} else {{ 0 }}", r[ci], a, r[bi]),
            Eqri => format!("{} = if {} == {} {{ 1 }} else {{ 0 }}", r[ci], r[ai], b),
            Eqrr => format!("{} = if {} == {} {{ 1 }} else {{ 0 }}", r[ci], r[ai], r[bi])
        }
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}, {:2}, {:2}, {:2}", self.opc, self.inp_a, self.inp_b, self.out_c)
    }
}

struct Device {
    registers: [u32; 6],
    ib: usize,
    ip: u32
}

impl Device {
    fn new(ib: usize) -> Device {
        Device { registers: [0; 6], ib: ib, ip: 0 }
    }

    fn exec(&mut self, ins: Instruction) {
        use OpCode::*;
        let r = &mut self.registers;
        let a = ins.inp_a;
        let ai = a as usize;
        let b = ins.inp_b;
        let bi = b as usize;
        let c = ins.out_c;
        let ci = c as usize;
        r[self.ib] = self.ip;
        match ins.opc {
            Addr => r[ci] = r[ai] + r[bi],
            Addi => r[ci] = r[ai] + b,
            Mulr => r[ci] = r[ai] * r[bi],
            Muli => r[ci] = r[ai] * b,
            Banr => r[ci] = r[ai] & r[bi],
            Bani => r[ci] = r[ai] & b,
            Borr => r[ci] = r[ai] | r[bi],
            Bori => r[ci] = r[ai] | b,
            Setr => r[ci] = r[ai],
            Seti => r[ci] = a,
            Gtir => r[ci] = if a > r[bi] { 1 } else { 0 },
            Gtri => r[ci] = if r[ai] > b { 1 } else { 0 },
            Gtrr => r[ci] = if r[ai] > r[bi] { 1 } else { 0 },
            Eqir => r[ci] = if a == r[bi] { 1 } else { 0 },
            Eqri => r[ci] = if r[ai] == b { 1 } else { 0 },
            Eqrr => r[ci] = if r[ai] == r[bi] { 1 } else { 0 }
        }
        self.ip = r[self.ib] + 1;
    }

    fn run(&mut self, program: &Vec<Instruction>) {
        self.run_to(program.len() as u32, program);
    }

    fn run_to(&mut self, to_ip: u32, program: &Vec<Instruction>) {
        while self.ip != to_ip && self.ip < program.len() as u32 {
            let ins = program[self.ip as usize];
            self.exec(ins);
        }
    }
}

#[cfg(feature = "print")]
fn print_instructions(instructions: &Vec<Instruction>, ipb: usize) {
    println!("Instructions:");
    println!("Ia OpCode A   B   C      Explained");
    for (i, ins) in instructions.iter().enumerate() {
        println!("{:2} {}  =>  {}", i, ins, ins.explain(ipb));
    }
}

// Re-implementation of the code found in the input
fn input_reimpl(a: u32) -> u32 {
    // Disassembly of input with comments
    //
    // Ia OpCode A   B   C      Explained
    //  0 Addi,  4, 16,  4  =>  e(ip) = e(ip) + 16               // Goto 17
    //  1 Seti,  1,  5,  1  =>  b = 1
    //  2 Seti,  1,  7,  3  =>  d = 1
    //  3 Mulr,  1,  3,  5  =>  f = b * d
    //  4 Eqrr,  5,  2,  5  =>  f = if f == c { 1 } else { 0 }
    //  5 Addr,  5,  4,  4  =>  e(ip) = f + e(ip)                // Goto 7 if f == c
    //  6 Addi,  4,  1,  4  =>  e(ip) = e(ip) + 1                // Goto 8 if f != c
    //  7 Addr,  1,  0,  0  =>  a = b + a
    //  8 Addi,  3,  1,  3  =>  d = d + 1
    //  9 Gtrr,  3,  2,  5  =>  f = if d > c { 1 } else { 0 }
    // 10 Addr,  4,  5,  4  =>  e(ip) = e(ip) + f                // Goto 12 if d > c
    // 11 Seti,  2,  4,  4  =>  e(ip) = 2                        // Goto 3 if d <= c
    // 12 Addi,  1,  1,  1  =>  b = b + 1
    // 13 Gtrr,  1,  2,  5  =>  f = if b > c { 1 } else { 0 }
    // 14 Addr,  5,  4,  4  =>  e(ip) = f + e(ip)                // Goto 16 if b > c (Effectively Exit)
    // 15 Seti,  1,  5,  4  =>  e(ip) = 1                        // Goto 2 if b <= c
    // 16 Mulr,  4,  4,  4  =>  e(ip) = e(ip) * e(ip)            // Goto 257 (Exit)
    // 17 Addi,  2,  2,  2  =>  c = c + 2
    // 18 Mulr,  2,  2,  2  =>  c = c * c
    // 19 Mulr,  4,  2,  2  =>  c = e(ip) * c
    // 20 Muli,  2, 11,  2  =>  c = c * 11
    // 21 Addi,  5,  2,  5  =>  f = f + 2
    // 22 Mulr,  5,  4,  5  =>  f = f * e(ip)
    // 23 Addi,  5, 18,  5  =>  f = f + 18
    // 24 Addr,  2,  5,  2  =>  c = c + f
    // 25 Addr,  4,  0,  4  =>  e(ip) = e(ip) + a
    // 26 Seti,  0,  6,  4  =>  e(ip) = 0                        // Goto 1 if a == 0
    // 27 Setr,  4,  3,  5  =>  f = e(ip)
    // 28 Mulr,  5,  4,  5  =>  f = f * e(ip)
    // 29 Addr,  4,  5,  5  =>  f = e(ip) + f
    // 30 Mulr,  4,  5,  5  =>  f = e(ip) * f
    // 31 Muli,  5, 14,  5  =>  f = f * 14
    // 32 Mulr,  5,  4,  5  =>  f = f * e(ip)
    // 33 Addr,  2,  5,  2  =>  c = c + f
    // 34 Seti,  0,  2,  0  =>  a = 0
    // 35 Seti,  0,  6,  4  =>  e(ip) = 0                        // Goto 1

    let (mut a, mut b, mut c, mut d, e, mut f) = (a, 0, 0, 0, 0, 0);

    // Setup (starts at instruction adress 17)
    c += 2;
    c *= c;
    c *= 19;
    c *= 11;
    f += 2;
    f *= 22;
    f += 18;
    c += f;
    if a == 1 {
        f = 27;
        f *= 28;
        f += 29;
        f *= 30;
        f *= 14;
        f *= 32;
        c += f;
        a = 0;
    }

    // Main iteration (starts at instruction adress 1)
    // Calculates the sum of all divisors for c
    b = 1;
    d = 1;
    loop {
        f = b * d;
        if f == c {
            a += b;
        }
        d = d + 1;
        if d > c {
            b = b + 1;
            if b > c {
                break;
            }
            d = 1;
        }
    }
    a
}

// The main iteration in the input can be solved much quicker like this
fn sum_divisors(n: u32) -> u32 {
    (1..=n).filter(|i| n % i == 0).sum()
}

fn part1(input: &Vec<String>) -> u32 {
    let (ipb, instructions) = parse_input(input);
    #[cfg(feature = "print")] print_instructions(&instructions, ipb);

    let mut d = Device::new(ipb);
    d.run(&instructions);

    d.registers[0]
 }

fn part2(input: &Vec<String>) -> u32 {
    let (ipb, instructions) = parse_input(input);
    #[cfg(feature = "print")] print_instructions(&instructions, ipb);

    let mut d = Device::new(ipb);
    d.registers[0] = 1;

    // Let the program run til it reaches ip=1
    d.run_to(1, &instructions);

    // Now we can assume the value to use is in register 3 (C)
    let value = d.registers[2];

    let fast = true;
    if fast {
        sum_divisors(value)
    } else {
        input_reimpl(1)
    }
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
    let f = File::open("src/day19/input")?;
    let f = BufReader::new(f);
    Ok(f.lines().map(|l| l.unwrap()).collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str =
       "#ip 0
        seti 5 0 1
        seti 6 0 2
        addi 0 1 0
        addr 1 2 3
        setr 1 0 0
        seti 8 0 4
        seti 9 0 5";

    fn as_input(s: &str) -> Vec<String> {
        s.split('\n').map(|s| s.trim().into()).collect()
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&as_input(INPUT)), 6);
    }

    #[test]
    fn test_sum_divisors() {
        assert_eq!(sum_divisors(10), 1 + 2 + 5 + 10);
        assert_eq!(sum_divisors(33), 1 + 3 + 11 + 33);
    }
}