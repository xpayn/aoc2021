use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> Vec<u16> {
    input
        .trim()
        .split(",")
        .map(|n| n.parse().unwrap())
        .collect::<Vec<_>>()
}

fn compute_score(input: &[u16], pivot: u16) -> usize {
    input
        .iter()
        .map(|pos| (*pos as i16 - pivot as i16).abs() as usize)
        .sum()
}

fn compute_score2(input: &[u16], pivot: u16) -> usize {
    input
        .iter()
        .map(|pos| {
            let n = (*pos as i16 - pivot as i16).abs() as usize;
            n * (n + 1) / 2
        })
        .sum()
}

#[aoc(day7, part1)]
pub fn solve_part1(input: &[u16]) -> usize {
    input
        .iter()
        .map(|pos| compute_score(input, *pos))
        .min()
        .unwrap()
}

#[aoc(day7, part2)]
pub fn solve_part2(input: &[u16]) -> usize {
    let max_pos = *input.iter().max().unwrap() as usize;
    (0..max_pos)
        .map(|pos| compute_score2(input, pos as u16))
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn d7_part1() {
        let input = input_generator("16,1,2,0,4,2,7,1,2,14");
        assert_eq!(solve_part1(input.as_slice()), 37);
    }
    #[test]
    fn d7_part2() {
        let input = input_generator("16,1,2,0,4,2,7,1,2,14");
        assert_eq!(solve_part2(input.as_slice()), 168);
    }
}
