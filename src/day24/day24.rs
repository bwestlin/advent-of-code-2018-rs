extern crate regex;
#[macro_use] extern crate lazy_static;
extern crate utils;

use std::cmp;
use std::collections::HashSet;
use std::error::Error;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use regex::Regex;
use utils::*;

#[derive(PartialEq, Eq, Clone, Debug)]
struct Group {
    n_units: i32,
    hp: i32,
    attack_d: i32,
    attack_boost: i32,
    attack_t: String,
    initiative: i32,
    weaknesses: HashSet<String>,
    immunities: HashSet<String>
}

impl Group {
    fn parse(input: &str) -> Group {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^(\d+)\s.+\s(\d+)\s.+\((.+)\).+\s(\d+)\s(\w+)\s.+\s(\d+)$").unwrap();
            static ref RE2: Regex = Regex::new(r"^(\d+)\s.+\s(\d+)\s.+(with).+\s(\d+)\s(\w+)\s.+\s(\d+)$").unwrap();
        }
        let caps = if RE.is_match(input) { RE.captures(input).unwrap() } else { RE2.captures(input).unwrap() };
        let geti = |idx| caps.get(idx).unwrap().as_str().parse::<i32>().unwrap();
        let gets = |idx| caps.get(idx).unwrap().as_str().to_string();

        let mut weaknesses: HashSet<String> = HashSet::new();
        let mut immunities: HashSet<String> = HashSet::new();
        for e in gets(3).split(';').map(|s| s.trim()) {
            if e.starts_with("weak to ") {
                weaknesses = e[8..].split(',').map(|s| s.trim().to_string()).collect();
            } else if !e.starts_with("with") {
                immunities = e[10..].split(',').map(|s| s.trim().to_string()).collect();
            }
        }

        Group {
            n_units: geti(1),
            hp: geti(2),
            attack_d: geti(4),
            attack_boost: 0,
            attack_t: gets(5),
            initiative: geti(6),
            weaknesses: weaknesses,
            immunities: immunities
        }
    }

    fn effective_power(&self) -> i32 {
        self.n_units * (self.attack_d + self.attack_boost)
    }

    fn max_damage_to(&self, other: &Group) -> i32 {
        let mul =
            if other.immunities.contains(&self.attack_t) { 0 }
            else if other.weaknesses.contains(&self.attack_t) { 2 }
            else { 1 };

        self.effective_power() * mul
    }

    fn take_damage(&mut self, damage: i32) {
        let units_lost = damage / self.hp;
        self.n_units = cmp::max(0, self.n_units - units_lost);
    }

    fn beaten(&self) -> bool {
        self.n_units <= 0
    }
}

#[derive(Clone, Debug)]
enum ArmyType {
    ImmuneSystem, Infection
}

#[derive(Clone, Debug)]
struct Army {
    t: ArmyType,
    groups: Vec<Group>
}

impl Army {
    fn select_targets(&self, targets: &Vec<Group>) -> Vec<(usize, usize)> {
        let mut gs: Vec<_> = self.groups.iter().enumerate().collect();
        let mut t_taken: HashSet<usize> = HashSet::new();
        let mut result: Vec<(usize, usize)> = vec![];

        gs.sort_by(|(_, a), (_, b)| b.effective_power().cmp(&a.effective_power()).then(b.initiative.cmp(&a.initiative)));

        for (i, g) in gs {
            let mut ts: Vec<_> = targets.iter().enumerate()
                .filter(|(i, _)| !targets[*i].beaten() && !t_taken.contains(i))
                .collect();
            if ts.len() == 0 {
                continue;
            }
            ts.sort_by(|(_, a), (_, b)| {
                g.max_damage_to(b).cmp(&g.max_damage_to(a))
                    .then(b.effective_power().cmp(&a.effective_power()))
                    .then(b.initiative.cmp(&a.initiative))
            });

            let (ti, tg) = ts[0];
            if g.max_damage_to(tg) == 0 {
                if ts.iter().map(|(_, tg)| g.max_damage_to(tg)).sum::<i32>() != 0 {
                    panic!("Something is wrong with target sorting!");
                }
                continue;
            }
            result.push((i, ti));
            t_taken.insert(ti);

            #[cfg(feature = "print")] for (ti, tg) in ts {
                println!("{:?} group {} would deal defending group {} {} damage", self.t, i + 1, ti + 1, g.max_damage_to(tg));
            }
        }
        result
    }

    fn beaten(&self) -> bool {
        self.groups.iter().all(|g| g.beaten())
    }

    fn units_left(&self) -> i32 {
        self.groups.iter().map(|g| g.n_units).sum()
    }

    fn boost(&mut self, boost: i32) {
        for i in 0..self.groups.len() {
            self.groups[i].attack_boost = boost;
        }
    }
}

fn parse_input(input: &Vec<String>) -> (Army, Army) {
    let mut im_grps = vec![];
    let mut in_grps = vec![];

    let mut in_im = true;
    let mut i = 1;
    while i < input.len() {
        if input[i].len() == 0 {
            i += 2;
            in_im = false;
        }
        if in_im {
            im_grps.push(Group::parse(&input[i]));
        } else {
            in_grps.push(Group::parse(&input[i]));
        }
        i += 1;
    }

    ( Army { t: ArmyType::ImmuneSystem, groups: im_grps }, Army { t: ArmyType::Infection, groups: in_grps })
}

fn fight(armies: [&mut Army; 2]) {
    #[cfg(feature = "print")] {
        println!("{:?}:", armies[0].t);
        for i in 0..armies[0].groups.len() {
            if armies[0].groups[i].beaten() {
                continue;
            }
            println!("Group {} contains {} units", i + 1, armies[0].groups[i].n_units);
        }

        println!("{:?}:", armies[1].t);
        for i in 0..armies[1].groups.len() {
            if armies[1].groups[i].beaten() {
                continue;
            }
            println!("Group {} contains {} units", i + 1, armies[1].groups[i].n_units);
        }
        println!();
    }

    let mut attacks: Vec<_> = (0..armies.len()).zip((0..armies.len()).cycle().skip(1))
        .flat_map(|(aai, dai)| {
            armies[aai].select_targets(&armies[dai].groups).iter()
                .map(|(agi, dgi)| ((aai, *agi), (dai, *dgi)))
                .collect::<Vec<_>>()
        })
        .collect();

    attacks.sort_by(|((aai, agi), _), ((bai, bgi), _)| {
        armies[*bai].groups[*bgi].initiative.cmp(&armies[*aai].groups[*agi].initiative)
    });

    #[cfg(feature = "print")] println!();

    for ((aai, agi), (dai, dgi)) in attacks {
        let _dgu_bef = armies[dai].groups[dgi].n_units;
        let damage_dealt = armies[aai].groups[agi].max_damage_to(&armies[dai].groups[dgi]);
        armies[dai].groups[dgi].take_damage(damage_dealt);
        #[cfg(feature = "print")] {
            println!("{:?} group {} attacks defending group {}, killing {} units", armies[aai].t, agi + 1, dgi + 1, _dgu_bef - armies[dai].groups[dgi].n_units);
        }
    }
}

fn part1(input: &Vec<String>) -> i32 {
    let (mut imsys, mut infec) = parse_input(input);

    for _r in 0.. {
        #[cfg(feature = "print")] {
            println!("-----------------------------------------------------------------");
            println!("Round: {}", _r + 1);
        }
        fight([&mut infec, &mut imsys]);

        if imsys.beaten() || infec.beaten() {
            break;
        }
    }

    if imsys.beaten() { infec.units_left() } else { imsys.units_left() }
}

fn part2(input: &Vec<String>) -> i32 {
    let (imsys, infec) = parse_input(input);

    let mut units_left = 0;
    for boost in 1.. {
        let mut ims = imsys.clone();
        let mut inf = infec.clone();
        ims.boost(boost);
        #[cfg(feature = "print")] println!("Trying boost {}", boost);

        let mut tie = false;
        for _ in 0.. {
            let units_before = inf.units_left() + ims.units_left();
            fight([&mut inf, &mut ims]);
            if units_before == inf.units_left() + ims.units_left() {
                tie = true;
                break;
            }

            if ims.beaten() || inf.beaten() {
                break;
            }
        }

        if !ims.beaten() && !tie {
            units_left = ims.units_left();
            break;
        }
    }
    units_left
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
    let f = File::open("src/day24/input")?;
    let f = BufReader::new(f);
    Ok(f.lines().map(|l| l.unwrap()).collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str =
       "Immune System:
        17 units each with 5390 hit points (weak to radiation, bludgeoning) with an attack that does 4507 fire damage at initiative 2
        989 units each with 1274 hit points (immune to fire; weak to bludgeoning, slashing) with an attack that does 25 slashing damage at initiative 3

        Infection:
        801 units each with 4706 hit points (weak to radiation) with an attack that does 116 bludgeoning damage at initiative 1
        4485 units each with 2961 hit points (immune to radiation; weak to fire, cold) with an attack that does 12 slashing damage at initiative 4";

    fn as_input(s: &str) -> Vec<String> {
        s.split('\n').map(|s| s.trim().into()).collect()
    }

    #[test]
    fn test_parse_group() {
        let input = "18 units each with 729 hit points (weak to fire; immune to cold, slashing) with an attack that does 8 radiation damage at initiative 10";
        let g = Group::parse(input);
        assert_eq!(g, Group {
            n_units: 18,
            hp: 729,
            attack_d: 8,
            attack_boost: 0,
            attack_t: "radiation".into(),
            initiative: 10,
            weaknesses: ["fire"].iter().map(|s| s.to_string()).collect(),
            immunities: ["cold", "slashing"].iter().map(|s| s.to_string()).collect()
        });
        let input = "6799 units each with 3314 hit points with an attack that does 4 radiation damage at initiative 16";
        let g = Group::parse(input);
        assert_eq!(g, Group {
            n_units: 6799,
            hp: 3314,
            attack_d: 4,
            attack_boost: 0,
            attack_t: "radiation".into(),
            initiative: 16,
            weaknesses: HashSet::new(),
            immunities: HashSet::new()
        });
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&as_input(INPUT)), 5216);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&as_input(INPUT)), 51);
    }
}
