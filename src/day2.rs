use aoc_runner_derive::{aoc, aoc_generator};
use std::ops::Add;

pub enum Command {
    Forward(u8),
    Down(u8),
    Up(u8),
}

#[derive(Default)]
struct Position {
    horizontal: u32,
    depth: u32,
}

impl Add<&Command> for Position {
    type Output = Position;

    fn add(self, _rhs: &Command) -> Position {
        match _rhs {
            Command::Forward(x) => Position {
                horizontal: self.horizontal + *x as u32,
                depth: self.depth,
            },
            Command::Down(x) => Position {
                horizontal: self.horizontal,
                depth: self.depth + *x as u32,
            },
            Command::Up(x) => Position {
                horizontal: self.horizontal,
                depth: self.depth - *x as u32,
            },
        }
    }
}

#[derive(Default)]
struct Position2 {
    horizontal: u32,
    depth: u32,
    aim: u32,
}

impl Add<&Command> for Position2 {
    type Output = Position2;

    fn add(self, _rhs: &Command) -> Position2 {
        match _rhs {
            Command::Forward(x) => Position2 {
                horizontal: self.horizontal + *x as u32,
                depth: self.depth + self.aim * *x as u32,
                aim: self.aim,
            },
            Command::Down(x) => Position2 {
                horizontal: self.horizontal,
                depth: self.depth,
                aim: self.aim + *x as u32,
            },
            Command::Up(x) => Position2 {
                horizontal: self.horizontal,
                depth: self.depth,
                aim: self.aim - *x as u32,
            },
        }
    }
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Command> {
    input
        .lines()
        .map(|l| {
            let command: Vec<&str> = l.trim().split(" ").collect();
            match command[0] {
                "forward" => Command::Forward(command[1].parse().unwrap()),
                "down" => Command::Down(command[1].parse().unwrap()),
                "up" => Command::Up(command[1].parse().unwrap()),
                _ => unreachable!(),
            }
        })
        .collect()
}

pub fn solve<'a, T: Add<&'a Command, Output = T> + Default>(input: &'a [Command]) -> T {
    input
        .iter()
        .fold(T::default(), |acc, command| acc + command)
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &[Command]) -> u64 {
    let position = solve::<Position>(input);
    (position.horizontal * position.depth).into()
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &[Command]) -> u64 {
    let position = solve::<Position2>(input);
    (position.horizontal * position.depth).into()
}
