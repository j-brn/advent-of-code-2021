use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
fn input_generator(input: &str) -> Vec<u32> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

#[aoc(day1, part1)]
pub fn part_one(measurements: &[u32]) -> usize {
    measurements.windows(2).filter(|w| w[0] < w[1]).count()
}

#[aoc(day1, part2)]
pub fn part_two(measurements: &[u32]) -> usize {
    let accumulated_measurements = measurements
        .windows(3)
        .map(|w| w.iter().sum())
        .collect::<Vec<u32>>();

    part_one(&accumulated_measurements)
}

#[cfg(test)]
mod tests {
    use crate::day1::{part_one, part_two};

    const MEASUREMENTS: &[u32] = &[199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

    #[test]
    fn test_part_one() {
        assert_eq!(7, part_one(MEASUREMENTS));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(5, part_two(MEASUREMENTS));
    }
}
