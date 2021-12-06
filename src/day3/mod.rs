use aoc_runner_derive::{aoc, aoc_generator};
use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;
use std::num::ParseIntError;

#[aoc_generator(day3)]
fn input_generator(input: &str) -> Result<Vec<u32>, ParseIntError> {
    input
        .lines()
        .map(|line| u32::from_str_radix(line, 2))
        .collect()
}

fn get_most_common_bit(index: usize, numbers: &[u32]) -> bool {
    let ones = numbers
        .iter()
        .copied()
        .filter(|&number| number & (1 << index) != 0)
        .count();

    ones >= numbers.len() / 2
}

fn filter_by_common_bit(index: usize, most_common: bool, numbers: &[u32]) -> Vec<u32> {
    let wanted_bit = get_most_common_bit(index, numbers);
    let wanted_bit = if most_common { wanted_bit } else { !wanted_bit };

    numbers
        .iter()
        .copied()
        .filter(|&number| ((number & 1 << index) != 0) == wanted_bit)
        .collect()
}

fn get_bit_counts<const LEN: usize>(numbers: &[u32]) -> [(usize, usize); LEN] {
    numbers.iter().fold([(0, 0); LEN], |counts, number| {
        (0..LEN).fold(counts, |mut counts, index| {
            let (zeroes, ones) = counts[index];

            counts[index] = match number & (1u32 << index) {
                0 => (zeroes + 1, ones),
                _ => (zeroes, ones + 1),
            };

            counts
        })
    })
}

#[aoc(day3, part1)]
fn part_one(numbers: &[u32]) -> u32 {
    let bit_counts = get_bit_counts::<12>(numbers);

    let gamma = bit_counts
        .iter()
        .enumerate()
        .fold(0u32, |mut gamma, (index, &(zeroes, ones))| {
            if ones > zeroes {
                gamma |= 1u32 << index
            }

            gamma
        });

    let epsilon = gamma ^ 2u32.pow(12) - 1;

    gamma * epsilon
}

#[aoc(day3, part2)]
fn part_two(numbers: &[u32]) -> u32 {
    let oxygen_numbers = (0..12)
        .rev()
        .fold_while(
            numbers.to_vec(),
            |oxygen_numbers, index| match oxygen_numbers.len() {
                1 => Done(oxygen_numbers),
                _ => Continue(filter_by_common_bit(index, true, &oxygen_numbers)),
            },
        )
        .into_inner();

    let co2_numbers = (0..12)
        .rev()
        .fold_while(numbers.to_vec(), |co2_numbers, index| {
            match co2_numbers.len() {
                1 => Done(co2_numbers),
                _ => Continue(filter_by_common_bit(index, false, &co2_numbers)),
            }
        })
        .into_inner();

    oxygen_numbers[0] * co2_numbers[0]
}
