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

    d.run_to(1, &instructions);

    // Assume value is in register 3 (C)
    sum_divisors(d.registers[2])
}

fn sum_divisors(n: u32) -> u32 {
    (1..=n).filter(|i| n % i == 0).sum()
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