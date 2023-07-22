/// Wise elf says...
///   The first column of your input is what the opponent will play;
///    A for Rock
///    B for Paper
///    C for Scissors
///   The second column is what we should play in response.
///    X for Rock
///    Y for Paper
///    Z for Scissors
/// Now we calculate what our score ends up being.
pub fn part_one(input: &str) -> Option<u32> {
    let rounds = input.trim_end().split('\n');
    Some(rounds.map(play_round).sum())
}

#[derive(Debug, PartialEq)]
pub enum Moves {
    Rock,
    Paper,
    Scissors,
}

pub fn rock_paper_scissors(us: &Moves, them: &Moves) -> u32 {
    match (us, them) {
        (Moves::Rock, Moves::Rock) => 3,
        (Moves::Rock, Moves::Paper) => 0,
        (Moves::Rock, Moves::Scissors) => 6,
        (Moves::Paper, Moves::Rock) => 6,
        (Moves::Paper, Moves::Paper) => 3,
        (Moves::Paper, Moves::Scissors) => 0,
        (Moves::Scissors, Moves::Rock) => 0,
        (Moves::Scissors, Moves::Paper) => 6,
        (Moves::Scissors, Moves::Scissors) => 3,
    }
}

pub fn moves_to_score(my_move: &Moves) -> u32 {
    match my_move {
        Moves::Rock => 1,
        Moves::Paper => 2,
        Moves::Scissors => 3,
    }
}

fn to_rock_paper_scissors(c: &char) -> Moves {
    match c {
        'X' => Moves::Rock,
        'Y' => Moves::Paper,
        'Z' => Moves::Scissors,
        'A' => Moves::Rock,
        'B' => Moves::Paper,
        'C' => Moves::Scissors,
        _ => panic!("Invalid Rock Paper Scissors choice!"),
    }
}

pub fn play_round(input: &str) -> u32 {
    let my_move = to_rock_paper_scissors(&input.chars().nth(2).unwrap());
    let their_move = to_rock_paper_scissors(&input.chars().nth(0).unwrap());
    rock_paper_scissors(&my_move, &their_move) + moves_to_score(&my_move)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rock_paper_scissors_outcomes() {
        assert_eq!(rock_paper_scissors(&Moves::Rock, &Moves::Scissors), 6);
        assert_eq!(rock_paper_scissors(&Moves::Paper, &Moves::Rock), 6);
        assert_eq!(rock_paper_scissors(&Moves::Scissors, &Moves::Paper), 6);

        assert_eq!(rock_paper_scissors(&Moves::Scissors, &Moves::Scissors), 3);
        assert_eq!(rock_paper_scissors(&Moves::Scissors, &Moves::Rock), 0);
    }

    #[test]
    fn test_moves_to_score() {
        assert_eq!(moves_to_score(&Moves::Rock), 1);
        assert_eq!(moves_to_score(&Moves::Paper), 2);
        assert_eq!(moves_to_score(&Moves::Scissors), 3);
    }

    #[test]
    fn test_to_rock_paper_scissors() {
        assert_eq!(to_rock_paper_scissors(&'A'), Moves::Rock);
        assert_eq!(to_rock_paper_scissors(&'X'), Moves::Rock);
    }

    #[test]
    fn test_win_round() {
        assert_eq!(play_round("A Y"), 8);
        assert_eq!(play_round("B Z"), 9);
        assert_eq!(play_round("C X"), 7);
    }

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), None);
    }
}
