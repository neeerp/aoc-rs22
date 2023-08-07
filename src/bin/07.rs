use std::{cell::RefCell, collections::HashMap, iter::Peekable, path::PathBuf, rc::Rc, str::Split};

use regex::Regex;

pub fn part_one(input: &str) -> Option<u32> {
    let all_dirs = execute(input.trim_end());

    let mut root = all_dirs.get(&PathBuf::from("/")).unwrap().borrow_mut();
    let mut results = root.resolve(&all_dirs);
    results.push(root.size);

    Some(results.iter().filter(|s| **s < 100_000).sum::<u32>())
}

pub fn execute(input: &str) -> HashMap<PathBuf, Rc<RefCell<Directory>>> {
    let mut execution = input.split('\n').peekable();
    let mut cwd = PathBuf::new();

    let mut all_dirs: HashMap<PathBuf, Rc<RefCell<Directory>>> = HashMap::new();

    while let Some(line) = execution.next() {
        let cmd = parse_command(line);
        match cmd {
            Command::LS => {
                execute_ls(&mut all_dirs, &mut execution, cwd.clone());
            }
            Command::CD(dir) => {
                cwd = execute_cd(dir, cwd.clone());
            }
        };
    }

    all_dirs
}

pub fn execute_ls(
    all_dirs: &mut HashMap<PathBuf, Rc<RefCell<Directory>>>,
    execution: &mut Peekable<Split<'_, char>>,
    cwd: PathBuf,
) {
    if let None = all_dirs.get(&cwd) {
        all_dirs.insert(
            cwd.clone(),
            Rc::new(RefCell::new(Directory::new(cwd.clone()))),
        );
    }

    let mut cur = all_dirs.get_mut(&cwd).unwrap().borrow_mut();

    while let Some(next_line) = execution.peek() {
        // Bail if we've reached the end of the ls output
        if is_command(*next_line) {
            break;
        }
        let line = execution.next().unwrap();

        // Parse ls output line and update the current directory
        let ls_output = parse_ls_output(line);
        match ls_output {
            LsOutput::Dir(dir) => {
                let mut child_path = cwd.clone();
                child_path.push(dir);
                cur.children.push(child_path);
            }
            LsOutput::File(_, size) => {
                cur.size += size;
                cur.file_sizes.push(size)
            }
        }
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[derive(Debug, PartialEq)]
pub enum Command<'a> {
    LS,
    CD(&'a str),
}

#[derive(Debug, PartialEq)]
pub struct Directory {
    pub path: PathBuf,
    pub size: u32,
    pub children: Vec<PathBuf>,
    pub file_sizes: Vec<u32>,
}

impl Directory {
    pub fn new(path: PathBuf) -> Self {
        Directory {
            path,
            size: 0,
            children: Vec::new(),
            file_sizes: Vec::new(),
        }
    }

    /// Resolve the size by resolving all children.
    pub fn resolve(&mut self, all_dirs: &HashMap<PathBuf, Rc<RefCell<Directory>>>) -> Vec<u32> {
        let mut resolved_sizes = vec![];
        for child_path in self.children.iter() {
            let mut child = all_dirs.get(child_path).unwrap().borrow_mut();

            let mut child_sizes = child.resolve(all_dirs);
            resolved_sizes.append(&mut child_sizes);

            self.size += child.size;
        }
        resolved_sizes.push(self.size);
        resolved_sizes
    }
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

pub fn execute_cd<'a>(cmd: &'a str, mut pwd: PathBuf) -> PathBuf {
    match cmd {
        "/" => PathBuf::from("/"),
        ".." => {
            pwd.pop();
            pwd
        }
        dir => {
            pwd.push(String::from(dir) + "/");
            pwd
        }
    }
}

pub fn is_command(line: &str) -> bool {
    line.chars().next().unwrap() == '$'
}

pub enum LsOutput<'a> {
    Dir(&'a str),
    File(&'a str, u32),
}

pub fn parse_ls_output<'a>(line: &'a str) -> LsOutput<'a> {
    let r = Regex::new(r"^(dir (?<dir>.+))|((?<size>\d+) (?<file>.+))$").expect("Oops!");
    let caps = r.captures(line).expect("Oops!");

    match caps {
        caps if matches!(caps.name("dir"), Some(_)) => {
            LsOutput::Dir(caps.name("dir").unwrap().as_str())
        }
        caps if matches!(caps.name("file"), Some(_)) => LsOutput::File(
            caps.name("file").unwrap().as_str(),
            caps.name("size").unwrap().as_str().parse().unwrap(),
        ),
        _ => panic!("Oops!"),
    }
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

        pwd = execute_cd("/", pwd);
        assert_eq!(pwd.as_path(), Path::new("/"));

        pwd = execute_cd("foo", pwd);
        assert_eq!(pwd.as_path(), Path::new("/foo/"));

        pwd = execute_cd("bar", pwd);
        assert_eq!(pwd.as_path(), Path::new("/foo/bar/"));

        pwd = execute_cd("..", pwd);
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
        assert!(!is_command("dir a"));
    }

    #[test]
    fn test_is_command() {
        assert!(is_command("$ ls"));
        assert!(is_command("$ cd foo"));
        assert!(is_command("$ sdgreawhgaikerojeroihgjeaioj"));
    }

    #[test]
    // #[ignore]
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
