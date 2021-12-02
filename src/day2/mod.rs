use aoc_runner_derive::{aoc, aoc_generator};
use std::num::ParseIntError;
use std::str::FromStr;
use thiserror::Error;

#[derive(Debug, Copy, Clone)]
enum Instruction {
    Forward(u32),
    Down(u32),
    Up(u32),
}

#[derive(Debug, Error)]
enum ParseInstructionError {
    #[error("Delimiter ' ' not found")]
    MissingDelimiter,
    #[error("Unknown instruction {0}")]
    UnknownInstruction(String),
    #[error("Failed to parse distance: {0}")]
    MalformedDistance(#[from] ParseIntError),
}

impl FromStr for Instruction {
    type Err = ParseInstructionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (instruction, distance) = s
            .split_once(" ")
            .ok_or(ParseInstructionError::MissingDelimiter)?;
        let distance = distance.parse()?;

        match instruction {
            "forward" => Ok(Instruction::Forward(distance)),
            "down" => Ok(Instruction::Down(distance)),
            "up" => Ok(Instruction::Up(distance)),
            _ => Err(ParseInstructionError::UnknownInstruction(
                instruction.to_owned(),
            )),
        }
    }
}

#[aoc_generator(day2)]
fn input_generator(input: &str) -> Result<Vec<Instruction>, ParseInstructionError> {
    input.lines().map(Instruction::from_str).collect()
}

#[aoc(day2, part1)]
fn part_one(instructions: &[Instruction]) -> u32 {
    let (horizontal, depth) = instructions.iter().fold(
        (0, 0),
        |(horizontal, depth), instruction| match instruction {
            Instruction::Forward(n) => (horizontal + n, depth),
            Instruction::Down(n) => (horizontal, depth + n),
            Instruction::Up(n) => (horizontal, depth - n),
        },
    );

    horizontal * depth
}

#[aoc(day2, part2)]
fn part_two(instructions: &[Instruction]) -> u32 {
    let (horizontal, depth, _aim) =
        instructions
            .iter()
            .fold(
                (0, 0, 0),
                |(horizontal, depth, aim), instruction| match instruction {
                    Instruction::Forward(n) => (horizontal + n, depth + aim * n, aim),
                    Instruction::Down(n) => (horizontal, depth, aim + n),
                    Instruction::Up(n) => (horizontal, depth, aim - n),
                },
            );

    horizontal * depth
}
