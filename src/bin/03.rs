use std::collections::HashSet;

const UPPER_A_ORD: u8 = 'A' as u8;
const LOWER_A_ORD: u8 = 'a' as u8;

pub fn part_one(input: &str) -> Option<usize> {
    let rucksacks = input.trim_end().split('\n');
    Some(rucksacks.map(find_priority_of).sum())
}

pub fn find_priority_of(rucksack: &str) -> usize {
    let (left, right) = rucksack.split_at(rucksack.len() / 2);
    to_priority(find_in_both(left, right))
}

pub fn find_in_both<'a>(left: &'a str, right: &'a str) -> char {
    let left_set: HashSet<char> = left.chars().collect();
    let right_set: HashSet<char> = right.chars().collect();

    left_set.intersection(&right_set).last().unwrap().clone()
}

pub fn to_priority(c: char) -> usize {
    if c.is_uppercase() {
        <u8 as Into<usize>>::into((c as u8) % UPPER_A_ORD) + 27
    } else {
        <u8 as Into<usize>>::into((c as u8) % LOWER_A_ORD) + 1
    }
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut rucksacks = input.trim_end().split('\n').peekable();
    let mut result = 0;
    while rucksacks.peek() != None {
        let one = rucksacks.next().unwrap();
        let two = rucksacks.next().unwrap();
        let three = rucksacks.next().unwrap();

        result += to_priority(find_in_three(one, two, three))
    }

    Some(result)
}

pub fn find_in_three<'a>(one: &'a str, two: &'a str, three: &'a str) -> char {
    let one_set: HashSet<char> = one.chars().collect();
    let two_set: HashSet<char> = two.chars().collect();
    let three_set: HashSet<char> = three.chars().collect();

    let one_two_set: HashSet<char> = one_set.intersection(&two_set).copied().collect();
    one_two_set.intersection(&three_set).last().unwrap().clone()
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_in_three_single() {
        assert_eq!(find_in_three("a", "a", "a"), 'a');
    }

    #[test]
    fn find_in_three_simple() {
        assert_eq!(find_in_three("abc", "cde", "cfg"), 'c');
    }

    #[test]
    fn find_in_three_complex() {
        assert_eq!(
            find_in_three(
                "vJrwpWtwJgWrhcsFMMfFFhFp",
                "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
                "PmmdzqPrVvPwwTWBwg"
            ),
            'r'
        );
    }

    #[test]
    fn find_in_both_single() {
        assert_eq!(find_in_both("a", "a"), 'a');
    }

    #[test]
    fn find_in_both_simple() {
        assert_eq!(find_in_both("abc", "cde"), 'c');
    }

    #[test]
    fn find_in_both_complex() {
        assert_eq!(find_in_both("vJrwpWtwJgWr", "hcsFMMfFFhFp"), 'p');
    }

    #[test]
    fn test_to_priority_lower() {
        assert_eq!(to_priority('a'), 1);
        assert_eq!(to_priority('z'), 26);
    }

    #[test]
    fn test_to_priority_upper() {
        assert_eq!(to_priority('A'), 27);
        assert_eq!(to_priority('Z'), 52);
    }

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(70));
    }
}
