use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Clone)]
struct Number {
    value: u8,
    marked: bool,
}

impl From<u8> for Number {
    fn from(v: u8) -> Self {
        Self {
            value: v,
            marked: false,
        }
    }
}
#[derive(Clone)]
struct Line(Vec<Number>);

impl Line {
    fn is_a_winner(&self) -> bool {
        self.0.iter().all(|n| n.marked)
    }

    fn sum_unmarked(&self) -> u32 {
        self.0
            .iter()
            .map(|n| if n.marked { 0 } else { n.value as u32 })
            .sum()
    }
}

#[derive(Default, Clone)]
pub struct Board {
    lines: Vec<Line>,
    complete: bool,
}

impl Board {
    fn get_score(&self, draw: u8) -> u32 {
        draw as u32
            * self
                .lines
                .iter()
                .map(|line| line.sum_unmarked())
                .sum::<u32>()
    }

    fn check_columns(&mut self, draw: u8) -> Option<u32> {
        for (i, n) in self.lines[0].0.iter().enumerate() {
            if n.marked && self.lines[1..].into_iter().all(|l| l.0[i].marked) {
                self.complete = true;
                return Some(self.get_score(draw));
            }
        }
        None
    }

    fn check_draw(&mut self, draw: u8) -> Option<u32> {
        for l in self.lines.iter_mut() {
            if let Some(mut number) = l.0.iter_mut().find(|x| x.value == draw) {
                number.marked = true;
            }
            if l.is_a_winner() {
                self.complete = true;
                return Some(self.get_score(draw));
            }
        }
        self.check_columns(draw)
    }
}

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> (Vec<u8>, Vec<Board>) {
    let mut lines = input.lines().into_iter();
    let draws: Vec<u8> = lines
        .next()
        .unwrap()
        .trim()
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect();

    let mut boards: Vec<Board> = vec![];
    for line in lines {
        if line.trim().is_empty() {
            boards.push(Board::default());
        } else {
            let board_line = Line(
                line.trim()
                    .split(" ")
                    .filter(|x| !x.is_empty())
                    .map(|x| x.parse::<u8>().unwrap().into())
                    .collect(),
            );
            boards.last_mut().unwrap().lines.push(board_line)
        }
    }

    (draws, boards)
}

#[aoc(day4, part1)]
pub fn solve_part1(input: &(Vec<u8>, Vec<Board>)) -> u32 {
    let mut boards = input.1.clone();
    for draw in input.0.iter() {
        for board in boards.iter_mut() {
            if let Some(score) = board.check_draw(*draw) {
                return score;
            }
        }
    }
    unreachable!()
}

#[aoc(day4, part2)]
pub fn solve_part2(input: &(Vec<u8>, Vec<Board>)) -> u32 {
    let mut boards = input.1.clone();
    for draw in input.0.iter() {
        let mut incomplete_boards: Vec<&mut Board> =
            boards.iter_mut().filter(|b| !b.complete).collect();
        let mut score = None;
        for board in incomplete_boards.iter_mut() {
            score = board.check_draw(*draw);
        }
        if incomplete_boards.len() == 1 && score.is_some() {
            return score.unwrap();
        }
    }
    unreachable!()
}
