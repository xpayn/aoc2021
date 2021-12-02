use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<u16> {
    input.lines().map(|l| l.trim().parse().unwrap()).collect()
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &[u16]) -> u16 {
    input.windows(2).map(|w| (w[1] > w[0]) as u16).sum()
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &[u16]) -> u16 {
    solve_part1(
        &input
            .windows(3)
            .map(|w| w.iter().sum())
            .collect::<Vec<u16>>(),
    )
}
