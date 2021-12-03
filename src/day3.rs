use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Vec<String> {
    input
        .lines()
        .map(|line| line.to_owned())
        .collect::<Vec<_>>()
}

fn toto(input: &[String]) -> Vec<i16> {
    input.into_iter().fold(Vec::new(), |mut acc, bits| {
        if acc.is_empty() {
            acc = bits
                .chars()
                .map(|b| if b == '0' { -1 } else { 1 })
                .collect();
        } else {
            for (i, b) in bits.chars().enumerate() {
                if b == '0' {
                    acc[i] -= 1
                } else {
                    acc[i] += 1
                }
            }
        }
        acc
    })
}

#[aoc(day3, part1)]
pub fn solve_part1(input: &[String]) -> u64 {
    let tata = toto(input);
    let mut gamma = 0;
    let mut epsilon = 0;
    for (i, count) in tata.iter().rev().enumerate() {
        if *count < 0 {
            epsilon += 1 << i;
        } else {
            gamma += 1 << i;
        }
    }
    gamma * epsilon
}

fn filter(input: &[String], index: usize, positive: bool) -> u64 {
    let tata = toto(input);
    let a = if tata[index] >= 0 && positive {
        input
            .into_iter()
            .cloned()
            .filter(|x| (**x).chars().nth(index) == Some('1'))
            .collect::<Vec<_>>()
    } else if tata[index] < 0 && positive {
        input
            .into_iter()
            .cloned()
            .filter(|x| (**x).chars().nth(index) == Some('0'))
            .collect::<Vec<_>>()
    } else if tata[index] < 0 && !positive {
        input
            .into_iter()
            .cloned()
            .filter(|x| (**x).chars().nth(index) == Some('1'))
            .collect::<Vec<_>>()
    } else if tata[index] >= 0 && !positive {
        input
            .into_iter()
            .cloned()
            .filter(|x| (**x).chars().nth(index) == Some('0'))
            .collect::<Vec<_>>()
    } else {
        unreachable!();
    };
    if a.len() > 1 {
        filter(a.as_slice(), index + 1, positive)
    } else {
        let mut ret = 0;
        for (i, b) in a[0].chars().rev().enumerate() {
            if b == '1' {
                ret += 1 << i;
            }
        }
        ret
    }
}

#[aoc(day3, part2)]
pub fn solve_part2(input: &[String]) -> u64 {
    let oxygen = dbg!(filter(input.clone(), 0, true));
    let co2 = dbg!(filter(input, 0, false));
    oxygen * co2
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn d3_part2() {
        let input = input_generator(
            "00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010",
        );
        assert_eq!(solve_part2(input.as_slice()), 230);
    }
}
