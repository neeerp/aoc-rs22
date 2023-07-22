use itertools::{rev, sorted};

/// Find the elf carrying the most calories; how much are they carrying?
pub fn part_one(input: &str) -> Option<u32> {
    let elves = input.trim_end().split("\n\n");
    elves.map(sum_elf).max()
}

pub fn sum_elf(input: &str) -> u32 {
    input
        .split('\n')
        .map(|n| -> u32 { n.parse::<u32>().unwrap() })
        .sum()
}

pub fn part_two(input: &str) -> Option<u32> {
    let elves = input.trim_end().split("\n\n");

    Some(rev(sorted(elves.map(sum_elf))).take(3).sum())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_elf() {
        let input = "1\n2\n3";
        assert_eq!(sum_elf(&input), 6);
    }

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(24000));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(45000));
    }
}
