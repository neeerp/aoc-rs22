use std::path::PathBuf;

use regex::Regex;

pub fn part_one(input: &str) -> Option<u32> {
    // Strategy might go something like this:
    //
    // Track where we currently are (cd commands)
    //
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[derive(Debug, PartialEq)]
pub enum Command<'a> {
    LS,
    CD(&'a str),
}

pub fn parse_command(line: &str) -> Command {
    let r = Regex::new(r"\$ ((?<ls>ls)|(?<cd>cd (?<arg>.+)))$").expect("Oops!");
    let caps = r.captures(line).expect("Oops!");

    match caps {
        caps if matches!(caps.name("ls"), Some(_)) => Command::LS,
        caps if matches!(caps.name("cd"), Some(_)) => {
            Command::CD(caps.name("arg").unwrap().as_str())
        }
        _ => panic!("Oops!"),
    }
}

pub fn execute_cd<'a>(cmd: Command, mut pwd: PathBuf) -> PathBuf {
    match cmd {
        Command::CD("/") => PathBuf::from("/"),
        Command::CD("..") => {
            pwd.pop();
            pwd
        }
        Command::CD(dir) => {
            pwd.push(String::from(dir) + "/");
            pwd
        }
        Command::LS => panic!("ERROR: called execute_cd with LS command!"),
    }
}

pub fn is_command(line: &str) -> bool {
    line.chars().next().unwrap() == '$'
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::*;

    #[test]
    fn test_execute_cd() {
        let mut pwd = PathBuf::new();

        pwd = execute_cd(Command::CD("/"), pwd);
        assert_eq!(pwd.as_path(), Path::new("/"));

        pwd = execute_cd(Command::CD("foo"), pwd);
        assert_eq!(pwd.as_path(), Path::new("/foo/"));

        pwd = execute_cd(Command::CD("bar"), pwd);
        assert_eq!(pwd.as_path(), Path::new("/foo/bar/"));

        pwd = execute_cd(Command::CD(".."), pwd);
        assert_eq!(pwd.as_path(), Path::new("/foo/"));
    }

    #[test]
    fn parse_cd() {
        assert_eq!(parse_command("$ cd foo"), Command::CD("foo"));
        assert_eq!(parse_command("$ cd .."), Command::CD(".."));
    }

    #[test]
    fn parse_ls() {
        assert_eq!(parse_command("$ ls"), Command::LS);
    }

    #[test]
    fn test_not_is_command() {
        assert!(!is_command("ls"));
    }

    #[test]
    fn test_is_command() {
        assert!(is_command("$ ls"));
        assert!(is_command("$ cd foo"));
        assert!(is_command("$ sdgreawhgaikerojeroihgjeaioj"));
    }

    #[test]
    #[ignore]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_one(&input), Some(95_437));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), None);
    }
}
