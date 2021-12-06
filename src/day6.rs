use std::fmt;

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Clone)]
pub struct Fish(u8);

impl Fish {
    fn new() -> Self {
        Self(8)
    }

    fn age(&mut self) -> bool {
        if self.0 == 0 {
            self.0 = 6;
            true
        } else {
            self.0 -= 1;
            false
        }
    }
}

impl fmt::Debug for Fish {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
        //f.debug_tuple("Fish").field(&self.0).finish()
    }
}

#[derive(Default, Clone, Copy)]
struct FishGroup {
    age: u8,
    count: u64,
}

impl fmt::Debug for FishGroup {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.age, self.count)
        //f.debug_tuple("Fish").field(&self.0).finish()
    }
}

struct Colony {
    groups: [FishGroup; 9],
}

impl Colony {
    fn new(fishes: &[Fish]) -> Self {
        let mut colony = Colony {
            groups: [FishGroup::default(); 9],
        };
        colony
            .groups
            .iter_mut()
            .enumerate()
            .for_each(|(i, group)| group.age = i as u8);
        fishes
            .iter()
            .for_each(|fish| colony.groups[fish.0 as usize].count += 1);
        colony
    }

    fn count(&self) -> u64 {
        self.groups.iter().map(|g| g.count).sum()
    }

    fn age(&mut self) {
        let mut birth_number = 0;
        self.groups.iter_mut().for_each(|g| {
            if g.age == 0 {
                g.age = 8;
                birth_number += g.count;
            } else {
                g.age -= 1;
            }
        });
        self.groups.iter_mut().find(|g| g.age == 6).unwrap().count += birth_number;
    }
}

impl fmt::Debug for Colony {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for g in self.groups {
            write!(f, "{:?} ", g)?;
        }
        writeln!(f, "")?;
        Ok(())
    }
}

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> Vec<Fish> {
    input
        .trim()
        .split(",")
        .map(|n| Fish(n.parse().unwrap()))
        .collect::<Vec<_>>()
}

#[aoc(day6, part1)]
pub fn solve_part1(input: &[Fish]) -> usize {
    let mut colony = input.to_vec();
    for _ in 0..80 {
        let mut new: u16 = 0;
        colony.iter_mut().for_each(|f: &mut Fish| {
            let gave_birth = (*f).age();
            if gave_birth {
                new += 1;
            }
        });
        colony.extend(vec![Fish::new(); new.into()]);
    }

    colony.len()
}

#[aoc(day6, part2)]
pub fn solve_part2(input: &[Fish]) -> u64 {
    let mut colony = Colony::new(input);
    println!("{:?}", colony);
    for _ in 0..256 {
        colony.age();
    }

    colony.count()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn d6_part1() {
        let input = input_generator("3,4,3,1,2");
        assert_eq!(solve_part1(input.as_slice()), 5934);
    }
    #[test]
    fn d6_part2() {
        let input = input_generator("3,4,3,1,2");
        assert_eq!(solve_part2(input.as_slice()), 26984457539);
    }
}
