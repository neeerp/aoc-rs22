use itertools::Itertools;

pub fn part_one(input: &str) -> Option<u32> {
    let (offset, _) = input
        .chars()
        .tuple_windows::<(char, char, char, char)>()
        .fold((4, false), |(offset, found), (a, b, c, d)| {
            if found {
                return (offset, found);
            }

            if [a, b, c, d].into_iter().all_unique() {
                (offset, true)
            } else {
                (offset + 1, false)
            }
        });
    Some(offset)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_one(&input), Some(7));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), None);
    }
}
