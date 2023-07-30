/// How many ranges fully contain the other?
pub fn part_one(input: &str) -> Option<usize> {
    Some(input.trim_end().split('\n').map(is_contained).sum())
}

pub fn is_contained(pairs: &str) -> usize {
    let (left, right) = parse_pairs(pairs);
    range_contains(left, right) as usize
}

/// Given two ranges l1-e1 and l2-e2, returns true iff one range is entirely
/// contained within the other.
pub fn range_contains(first: &str, second: &str) -> bool {
    let (l1, e1) = parse_range(first);
    let (l2, e2) = parse_range(second);

    (l1 <= l2 && e1 >= e2) || (l2 <= l1 && e2 >= e1)
}

/// Parses a range of the form i-j into two boundaries (i, j)
pub fn parse_range(range: &str) -> (usize, usize) {
    let boundaries = range.split('-');
    let lower: usize = boundaries.clone().nth(0).unwrap().parse().unwrap();
    let upper: usize = boundaries.clone().nth(1).unwrap().parse().unwrap();
    (lower, upper)
}

/// Parses a pair of the form X,Y into two string slices (X, Y)
pub fn parse_pairs(pairs: &str) -> (&str, &str) {
    let split = pairs.split(',');
    (split.clone().nth(0).unwrap(), split.clone().nth(1).unwrap())
}

/// How many pairs overlap at all?
pub fn part_two(input: &str) -> Option<usize> {
    Some(input.trim_end().split('\n').map(is_overlapping).sum())
}

pub fn is_overlapping(pairs: &str) -> usize {
    let (left, right) = parse_pairs(pairs);
    range_overlaps(left, right) as usize
}

pub fn range_overlaps(first: &str, second: &str) -> bool {
    let (l1, e1) = parse_range(first);
    let (l2, e2) = parse_range(second);

    // Boundaries touch
    if l1 == l2 || e1 == e2 || l1 == e2 || l2 == e1 {
        return true;
    }

    if l1 < l2 {
        e1 > l2
    } else {
        e2 > l1
    }
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_range_does_not_overlap() {
        assert!(!range_overlaps("1-4", "5-6"));
        assert!(!range_overlaps("5-6", "1-4"));
    }

    #[test]
    fn test_range_overlaps() {
        assert!(range_overlaps("1-5", "5-6"));
        assert!(range_overlaps("5-6", "1-5"));
    }

    #[test]
    fn test_contained_range_overlaps() {
        assert!(range_overlaps("1-5", "2-5"));
        assert!(range_overlaps("2-5", "1-5"));
    }

    #[test]
    fn test_range_contains() {
        assert!(range_contains("1-5", "2-5"));
    }

    #[test]
    fn test_range_does_not_contain() {
        assert!(!range_contains("1-5", "2-6"));
    }

    #[test]
    fn test_parse_range() {
        assert_eq!(parse_range("1-5"), (1, 5));
    }

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }
}
