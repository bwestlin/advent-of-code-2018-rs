extern crate regex;
#[macro_use] extern crate lazy_static;
extern crate utils;

use std::collections::BTreeMap;
use std::error::Error;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use regex::Regex;
use utils::*;

fn parse_input(input: &Vec<String>) -> (Vec<Sample>, Vec<[u32; 4]>) {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^\D*(\d+)\D+(\d+)\D+(\d+)\D+(\d+)\D*$").unwrap();
    }

    let get_vals = |s: &String| {
        let caps = RE.captures(s).unwrap();
        let get = |idx| caps.get(idx).unwrap().as_str().parse::<u32>().unwrap();
        [get(1), get(2), get(3), get(4)]
    };

    let mut samples: Vec<Sample> = vec![];
    let mut instructions: Vec<[u32; 4]> = vec![];

    let mut idx = 0;
    while idx < input.len() {
        let l = &input[idx];
        if l.starts_with("Before:") {
            samples.push(Sample {
                before: get_vals(l),
                instruction: get_vals(&input[idx + 1]),
                after: get_vals(&input[idx + 2])
            });
            idx += 3;
        } else if l.trim().len() > 0 {
            instructions.push(get_vals(l));

        }
        idx += 1;
    }
    (samples, instructions)
}

#[derive(Debug)]
struct Sample {
    before: [u32; 4],
    instruction: [u32; 4],
    after: [u32; 4]
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
enum OpCode {
    Addr, Addi, Mulr, Muli, Banr, Bani, Borr, Bori, Setr, Seti, Gtir, Gtri, Gtrr, Eqir, Eqri, Eqrr
}

impl OpCode {
    fn all() -> Vec<OpCode> {
        use OpCode::*;
        vec![Addr, Addi, Mulr, Muli, Banr, Bani, Borr, Bori, Setr, Seti, Gtir, Gtri, Gtrr, Eqir, Eqri, Eqrr]
    }
}

struct Instruction {
    opc: OpCode,
    inp_a: u32,
    inp_b: u32,
    out_c: u32
}

struct Device {
    registers: [u32; 4]
}

impl Device {
    fn new() -> Device {
        Device { registers: [0; 4] }
    }

    fn eval(&mut self, ins: Instruction) {
        let r = &mut self.registers;
        let a = ins.inp_a;
        let ai = a as usize;
        let b = ins.inp_b;
        let bi = b as usize;
        let c = ins.out_c;
        let ci = c as usize;
        match ins.opc {
            OpCode::Addr => r[ci] = r[ai] + r[bi],
            OpCode::Addi => r[ci] = r[ai] + b,
            OpCode::Mulr => r[ci] = r[ai] * r[bi],
            OpCode::Muli => r[ci] = r[ai] * b,
            OpCode::Banr => r[ci] = r[ai] & r[bi],
            OpCode::Bani => r[ci] = r[ai] & b,
            OpCode::Borr => r[ci] = r[ai] | r[bi],
            OpCode::Bori => r[ci] = r[ai] | b,
            OpCode::Setr => r[ci] = r[ai],
            OpCode::Seti => r[ci] = a,
            OpCode::Gtir => r[ci] = if a > r[bi] { 1 } else { 0 },
            OpCode::Gtri => r[ci] = if r[ai] > b { 1 } else { 0 },
            OpCode::Gtrr => r[ci] = if r[ai] > r[bi] { 1 } else { 0 },
            OpCode::Eqir => r[ci] = if a == r[bi] { 1 } else { 0 },
            OpCode::Eqri => r[ci] = if r[ai] == b { 1 } else { 0 },
            OpCode::Eqrr => r[ci] = if r[ai] == r[bi] { 1 } else { 0 }
        }
    }
}

fn part1(input: &Vec<String>) -> usize {
    let (samples, _) = parse_input(input);

    samples.iter()
        .fold(0, |acc, s| {
            let n_matching = OpCode::all().iter()
                .filter(|&oc| {
                    let mut d = Device { registers: s.before };
                    d.eval(Instruction {
                        opc: *oc,
                        inp_a: s.instruction[1],
                        inp_b: s.instruction[2],
                        out_c: s.instruction[3]
                    });
                    d.registers == s.after
                })
                .count();

            acc + if n_matching >= 3 { 1 } else { 0 }
        })
}

fn detect_opcodes(samples: &Vec<Sample>) -> BTreeMap<u32, OpCode> {

    // Calculate OpCode frequency per opcode-value
    let mut ocs_freq: BTreeMap<u32, BTreeMap<OpCode, i32>> = BTreeMap::new();
    for s in samples {
        let ocs_cnt = &mut *ocs_freq.entry(s.instruction[0]).or_insert(BTreeMap::new());
        for oc in OpCode::all() {
            let mut d = Device { registers: s.before };
            d.eval(Instruction {
                opc: oc,
                inp_a: s.instruction[1],
                inp_b: s.instruction[2],
                out_c: s.instruction[3]
            });

            if d.registers == s.after {
                *ocs_cnt.entry(oc).or_insert(0) += 1;
            }
        }
    }

    // Calculate OpCodes with highest frequency per opcode-value
    let mut ocs_hfreq: BTreeMap<u32, Vec<OpCode>> = BTreeMap::new();
    for i in ocs_freq.keys() {
        let freq = ocs_freq.get(i).unwrap();
        let mfreq = freq.iter().map(|(_, f)| *f).max().unwrap();
        let ocs: Vec<_> = freq.iter().filter(|(_, &f)| f == mfreq).map(|(oc, _)| *oc).collect();
        ocs_hfreq.insert(*i, ocs);
    }

    // Find opcode-value to OpCode matches by identifying those not already identified with only one match
    // in the given iteration
    let mut ocs_matches: BTreeMap<u32, OpCode> = BTreeMap::new();
    while ocs_matches.len() < 15 {
        for i in ocs_hfreq.keys() {
            let freqs = ocs_hfreq.get(i).unwrap();
            let matches: Vec<&OpCode> = freqs.iter().filter(|&oc| !ocs_matches.values().any(|moc| moc == oc)).collect();
            if matches.len() == 1 {
                ocs_matches.insert(*i, matches[0].to_owned());
            }
        }
    }

    #[cfg(feature = "print")] {
        println!("Opcode matches");
        for (i, oc) in ocs_matches.iter() {
            println!("{:2} = {:?}", i, oc);
        }
    }

    ocs_matches
}

fn part2(input: &Vec<String>) -> u32 {
    let (samples, instructions) = parse_input(input);
    let mut device = Device::new();

    let opcode_lookup = detect_opcodes(&samples);

    for i in instructions {
        let ins = Instruction {
            opc: *opcode_lookup.get(&i[0]).unwrap(),
            inp_a: i[1],
            inp_b: i[2],
            out_c: i[3]
        };
        device.eval(ins);
    }

    device.registers[0]
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
    let f = File::open("src/day16/input")?;
    let f = BufReader::new(f);
    Ok(f.lines().map(|l| l.unwrap()).collect())
}
