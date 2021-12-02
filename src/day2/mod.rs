use aoc_runner_derive::{aoc, aoc_generator};
use parse_display::{Display, FromStr, ParseError};
use std::str::FromStr;

#[derive(Debug, Copy, Clone, FromStr, Display)]
#[display("{direction} {distance}")]
struct Instruction {
    direction: Direction,
    distance: u32,
}

#[derive(Debug, Copy, Clone, FromStr, Display)]
#[display(style = "lowercase")]
enum Direction {
    Forward,
    Down,
    Up,
}

#[aoc_generator(day2)]
fn input_generator(input: &str) -> Result<Vec<Instruction>, ParseError> {
    input.lines().map(Instruction::from_str).collect()
}

#[aoc(day2, part1)]
fn part_one(instructions: &[Instruction]) -> u32 {
    let (horizontal, depth) = instructions.iter().fold(
        (0, 0),
        |(horizontal, depth),
         Instruction {
             direction,
             distance,
         }| match direction {
            Direction::Forward => (horizontal + distance, depth),
            Direction::Down => (horizontal, depth + distance),
            Direction::Up => (horizontal, depth - distance),
        },
    );

    horizontal * depth
}

#[aoc(day2, part2)]
fn part_two(instructions: &[Instruction]) -> u32 {
    let (horizontal, depth, _aim) = instructions.iter().fold(
        (0, 0, 0),
        |(horizontal, depth, aim),
         Instruction {
             direction,
             distance,
         }| match direction {
            Direction::Forward => (horizontal + distance, depth + aim * distance, aim),
            Direction::Down => (horizontal, depth, aim + distance),
            Direction::Up => (horizontal, depth, aim - distance),
        },
    );

    horizontal * depth
}
