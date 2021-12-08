use std::{
    collections::{HashMap, HashSet},
    ops::Sub,
};

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(PartialEq, Clone)]
struct Pattern(HashSet<char>);

impl Pattern {
    fn len(&self) -> usize {
        self.0.len()
    }

    fn contains(&self, other: &Pattern) -> bool {
        other.0.is_subset(&self.0)
    }

    fn contains_char(&self, c: &char) -> bool {
        self.0.contains(c)
    }
}

impl Sub<&Pattern> for Pattern {
    type Output = Pattern;

    fn sub(self, rhs: &Pattern) -> Self::Output {
        Pattern(
            self.0
                .difference(&rhs.0)
                .map(|x| *x)
                .collect::<HashSet<char>>(),
        )
    }
}

impl Sub<&Pattern> for &Pattern {
    type Output = Pattern;

    fn sub(self, rhs: &Pattern) -> Self::Output {
        Pattern(
            self.0
                .difference(&rhs.0)
                .map(|x| *x)
                .collect::<HashSet<char>>(),
        )
    }
}

#[derive(Debug, Clone)]
struct SegmentError;

impl TryInto<char> for Pattern {
    type Error = SegmentError;

    fn try_into(self) -> Result<char, Self::Error> {
        if self.0.len() == 1 {
            Ok(*self.0.iter().next().unwrap())
        } else {
            println!("Error: {:?}", self.0);
            Err(SegmentError)
        }
    }
}

#[derive(Default)]
pub struct Input {
    patterns: Vec<Pattern>,
    outputs: Vec<Pattern>,
}

fn parse_part(part: &str, sorted: bool) -> Vec<Pattern> {
    let mut ret = part
        .split(" ")
        .map(|s| {
            let mut l = s.to_owned().chars().collect::<HashSet<char>>();
            Pattern(l)
        })
        .collect::<Vec<_>>();
    if sorted {
        ret.sort_unstable_by(|a, b| a.len().cmp(&b.len()));
    }
    ret
}

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Vec<Input> {
    input
        .lines()
        .map(|line| {
            let parts = line.trim().split(" | ").collect::<Vec<_>>();
            Input {
                patterns: parse_part(parts[0], true),
                outputs: parse_part(parts[1], false),
            }
        })
        .collect::<Vec<_>>()
}

#[aoc(day8, part1)]
pub fn solve_part1(input: &[Input]) -> usize {
    input
        .iter()
        .map(|x| {
            x.outputs
                .iter()
                .filter(|s| match s.len() {
                    2 | 3 | 4 | 7 => true,
                    _ => false,
                })
                .count()
        })
        .sum()
}

#[aoc(day8, part2)]
pub fn solve_part2(input: &[Input]) -> usize {
    input
        .iter()
        .map(|i| {
            let mut digits: Vec<Option<Pattern>> = vec![None; 10];
            // patterns are sorted by size

            digits[1] = Some(i.patterns[0].clone()); // size 2 -> 1
            digits[4] = Some(i.patterns[2].clone()); // size 4 -> 4
            digits[7] = Some(i.patterns[1].clone()); // size 3 -> 7
            digits[8] = Some(i.patterns[9].clone()); // size 7 -> 8

            let top: char = (digits[7].as_ref().unwrap() - digits[1].as_ref().unwrap())
                .try_into()
                .unwrap();
            for p in i.patterns[3..6].iter() {
                // if size 5 (2,3,5) contains 1 -> 3
                if p.contains(digits[1].as_ref().unwrap()) {
                    digits[3] = Some(p.clone());
                    break;
                }
            }
            for p in i.patterns[6..9].iter() {
                // if size 6 (0,6,9) contains 3 -> 9
                if p.contains(&digits[3].as_ref().unwrap()) {
                    digits[9] = Some(p.clone());
                    break;
                }
            }
            let bottom_left: char = (digits[8].as_ref().unwrap() - digits[9].as_ref().unwrap())
                .try_into()
                .unwrap();
            for p in i.patterns[3..6].iter() {
                // if size 5 (2,3,5) contains bottom left -> 2
                if p.contains_char(&bottom_left) {
                    digits[2] = Some(p.clone());
                } else if *p != digits[3].clone().unwrap() {
                    digits[5] = Some(p.clone());
                }
            }
            let top_right: char = (digits[9].as_ref().unwrap() - digits[5].as_ref().unwrap())
                .try_into()
                .unwrap();
            for p in i.patterns[6..9].iter() {
                // if size 6 (0,6,9) does not contains top_right -> 6
                if !p.contains_char(&top_right) {
                    digits[6] = Some(p.clone());
                } else if *p != digits[9].clone().unwrap() {
                    digits[0] = Some(p.clone());
                }
            }
            i.outputs
                .iter()
                .rev()
                .enumerate()
                .map(|(i, o)| {
                    digits
                        .iter()
                        .position(|p| *o == p.clone().unwrap())
                        .unwrap()
                        * 10_usize.pow(i as u32)
                })
                .sum::<usize>()
        })
        .sum()
}
