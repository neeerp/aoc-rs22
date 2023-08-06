use itertools::Itertools;
use regex::Regex;

/// This was truly awful.
pub fn part_one(input: &str) -> Option<String> {
    let mut parts = input.trim_end().split("\n\n");

    let mut stacks = read_map(parts.next().unwrap());
    parts
        .next()
        .unwrap()
        .split('\n')
        .for_each(|instruction| apply_instruction(instruction, &mut stacks));

    Some(stacks.iter().map(|s| s.last().unwrap()).collect::<String>())
}

pub fn apply_instruction(instruction: &str, stacks: &mut Vec<Vec<char>>) {
    let instruction = parse_instruction(instruction);
    for _ in 0..instruction.qty {
        let popped = stacks.get_mut(instruction.from).unwrap().pop().unwrap();
        stacks.get_mut(instruction.to).unwrap().push(popped);
    }
}

pub fn read_map(map: &str) -> Vec<Vec<char>> {
    let tmp = map.split('\n').rev().skip(1).map(|line| {
        line.chars()
            .skip(1)
            .step_by(4)
            .map(|c| match c {
                ' ' => None,
                c => Some(c),
            })
            .collect_vec()
    });

    let length = tmp.clone().last().unwrap().iter().count();
    tmp.fold(vec![vec![]; length], |mut s: Vec<Vec<char>>, line| {
        s.iter_mut().zip(line).for_each(|(stack, ele)| {
            if let Some(c) = ele {
                stack.push(c)
            }
        });
        s
    })
}

#[derive(PartialEq, Debug)]
pub struct Instruction {
    pub qty: usize,
    pub from: usize,
    pub to: usize,
}

pub fn parse_instruction(instruction: &str) -> Instruction {
    let re = Regex::new(r"^move (?<qty>\d+) from (?<from>\d+) to (?<to>\d+)").expect("Oops");
    let caps = re
        .captures(instruction)
        .expect("Could not parse instruction");
    Instruction {
        qty: caps.name("qty").unwrap().as_str().parse().unwrap(),
        from: caps
            .name("from")
            .unwrap()
            .as_str()
            .parse::<usize>()
            .unwrap()
            - 1,
        to: caps.name("to").unwrap().as_str().parse::<usize>().unwrap() - 1,
    }
}

pub fn part_two(input: &str) -> Option<String> {
    let mut parts = input.trim_end().split("\n\n");

    let mut stacks = read_map(parts.next().unwrap());
    parts
        .next()
        .unwrap()
        .split('\n')
        .for_each(|instruction| apply_improved_instruction(instruction, &mut stacks));

    Some(stacks.iter().map(|s| s.last().unwrap()).collect::<String>())
}

pub fn apply_improved_instruction(instruction: &str, stacks: &mut Vec<Vec<char>>) {
    let instruction = parse_instruction(instruction);
    let mut holding = vec![];

    for _ in 0..instruction.qty {
        holding.push(stacks.get_mut(instruction.from).unwrap().pop().unwrap());
    }

    let to = stacks.get_mut(instruction.to).unwrap();
    holding.iter().rev().for_each(|e| to.push(e.clone()));
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_instruction() {
        let instruction = "move 1 from 2 to 3";
        let parsed_instruction = Instruction {
            qty: 1,
            from: 1,
            to: 2,
        };

        assert_eq!(parse_instruction(instruction), parsed_instruction);
    }

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), Some(String::from("CMZ")));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), Some(String::from("MCD")));
    }
}
