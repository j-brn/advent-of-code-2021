use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashSet;
use std::str::FromStr;

const ROWS: &'static [[usize; 5]] = &[
    // horizontal
    [0, 1, 2, 3, 4],
    [5, 6, 7, 8, 9],
    [10, 11, 12, 13, 14],
    [15, 16, 17, 18, 19],
    [20, 21, 22, 23, 24],
    // vertical
    [0, 5, 10, 15, 20],
    [1, 6, 11, 16, 21],
    [2, 7, 12, 17, 22],
    [3, 8, 13, 18, 23],
    [4, 9, 14, 19, 24],
    // diagonal
    [0, 6, 12, 18, 24],
    [20, 16, 12, 8, 4],
];

#[derive(Debug, Clone, Hash)]
struct Board {
    numbers: [u32; 25],
    marked: [bool; 25],
}

impl Board {
    pub fn finished(&self) -> bool {
        ROWS.iter()
            .any(|row| row.iter().all(|index| self.marked[*index]))
    }

    pub fn score(&self, last_number: u32) -> u32 {
        let sum_of_unmarked: u32 = self
            .marked
            .iter()
            .copied()
            .enumerate()
            .filter(|&(_index, marked)| !marked)
            .map(|(index, _marked)| self.numbers[index])
            .sum();

        sum_of_unmarked * last_number
    }

    pub fn mark_number(&mut self, number: u32) {
        if let Some((index, _)) = self
            .numbers
            .iter()
            .copied()
            .enumerate()
            .filter(|&(_index, num)| num == number)
            .next()
        {
            self.marked[index] = true;
        }
    }
}

impl FromStr for Board {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let numbers = s
            .lines()
            .map(|line| {
                line.split_whitespace()
                    .map(|part| part.parse::<u32>().unwrap())
            })
            .flatten()
            .enumerate()
            .fold([0; 25], |mut board, (index, number)| {
                board[index] = number;
                board
            });

        Ok(Board {
            numbers,
            marked: [false; 25],
        })
    }
}

fn play_until_first_win(numbers: &[u32], mut boards: Vec<Board>) -> (u32, Board) {
    for number in numbers {
        for board in boards.iter_mut() {
            board.mark_number(*number);

            if board.finished() {
                return (*number, board.clone());
            }
        }
    }

    unreachable!()
}

fn play_to_end(numbers: &[u32], mut boards: Vec<Board>) -> Vec<u32> {
    let mut scores = Vec::with_capacity(boards.len());
    let mut skip = HashSet::with_capacity(boards.len());

    for &number in numbers {
        for (id, board) in boards.iter_mut().enumerate() {
            if skip.contains(&id) {
                continue;
            }

            board.mark_number(number);

            if board.finished() {
                skip.insert(id);
                scores.push(board.score(number));
            }
        }
    }

    scores
}

#[aoc_generator(day4)]
fn input_generator(input: &str) -> (Vec<u32>, Vec<Board>) {
    let mut blocks = input.split("\n\n");

    let numbers = blocks
        .next()
        .unwrap()
        .split(',')
        .map(|part| part.parse::<u32>().unwrap())
        .collect();

    let boards = blocks.map(|s| Board::from_str(&s).unwrap()).collect();

    (numbers, boards)
}

#[aoc(day4, part1)]
fn part_one((numbers, boards): &(Vec<u32>, Vec<Board>)) -> u32 {
    let (winning_number, winning_board) = play_until_first_win(numbers, boards.clone());

    winning_board.score(winning_number)
}

#[aoc(day4, part2)]
fn part_two((numbers, boards): &(Vec<u32>, Vec<Board>)) -> u32 {
    let scores = play_to_end(numbers, boards.clone());
    *scores.last().unwrap()
}
