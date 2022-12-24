use anyhow::{anyhow, Result};
use std::{
    cell::RefCell,
    collections::HashMap,
    rc::{Rc, Weak},
    str::FromStr,
};

#[derive(Debug, PartialEq)]
enum Command {
    CdDown(String),
    CdUp,
    Ls,
}

impl FromStr for Command {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        return match s.split(" ").collect::<Vec<_>>()[..] {
            [_, "cd", ".."] => Ok(Command::CdUp),
            [_, "cd", s] => Ok(Command::CdDown(String::from_str(s)?)),
            [_, "ls"] => Ok(Command::Ls),
            _ => Err(anyhow!("Invalid Command input {s}")),
        };
    }
}

#[derive(Debug, PartialEq)]
enum File {
    RegularFile { size: usize, name: String },
    Directory { name: String },
}

impl FromStr for File {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        return match s.split(" ").collect::<Vec<_>>()[..] {
            ["dir", name] => Ok(File::Directory {
                name: name.to_string(),
            }),
            [size, name] => Ok(File::RegularFile {
                size: size.parse()?,
                name: name.to_string(),
            }),
            _ => Err(anyhow!("Invalid File input {s}")),
        };
    }
}

#[derive(Debug, PartialEq)]
enum ShellLine {
    InputLine(Command),
    OutputLine(File),
}

impl FromStr for ShellLine {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        if s.starts_with('$') {
            return Ok(ShellLine::InputLine(s.parse::<Command>()?));
        } else {
            return Ok(ShellLine::OutputLine(s.parse::<File>()?));
        }
    }
}

#[derive(Debug)]
struct FileSystemNode {
    parent: Option<Weak<RefCell<FileSystemNode>>>,
    dirs: HashMap<String, Rc<RefCell<FileSystemNode>>>,
    files: Vec<crate::File>,
}

impl FileSystemNode {
    fn new(parent: Rc<RefCell<FileSystemNode>>) -> Self {
        Self {
            parent: Some(Rc::downgrade(&parent)),
            dirs: HashMap::new(),
            files: vec![],
        }
    }
}

struct FileSystem {
    root: Rc<RefCell<FileSystemNode>>,
}

impl FileSystem {
    fn report(&self) -> Vec<(String, usize)> {
        fn go(
            node: &FileSystemNode,
            dir_name: &str,
            mut result: Vec<(String, usize)>,
        ) -> Vec<(String, usize)> {
            let files_size = node.files.iter().fold(0, |acc, x| match x {
                File::RegularFile { size, .. } => acc + size,
                _ => panic!("Unexpected file type"),
            });

            let mut dir_size = 0_usize;
            for (dir_name, dir) in &node.dirs {
                result = go(&dir.borrow(), dir_name, result);
                if let Some(last_dir) = result.last() {
                    dir_size += last_dir.1;
                }
            }

            let size = dir_size + files_size;
            result.push((dir_name.to_owned(), size));

            result
        }

        go(&self.root.borrow(), &"/".to_string(), vec![])
    }
}

struct Session {
    current_dir: Rc<RefCell<FileSystemNode>>,
}

impl Session {
    fn cd_up(mut self) -> Self {
        let new_current_dir = self
            .current_dir
            .borrow()
            .parent
            .as_ref()
            .expect("Parent should not be null")
            .upgrade()
            .expect("Parent should exist already");

        self.current_dir = new_current_dir;
        self
    }

    fn cd_down(mut self, dir: &str) -> Self {
        self.current_dir = Rc::clone(
            Rc::clone(&self.current_dir)
                .borrow_mut()
                .dirs
                .entry(dir.to_string())
                .or_insert_with(|| Rc::new(RefCell::new(FileSystemNode::new(self.current_dir)))),
        );
        self
    }

    fn add_file(self, file: File) -> Self {
        self.current_dir.borrow_mut().files.push(file);
        self
    }
}

fn main() -> Result<()> {
    let solution_1 = solve_part_one();

    println!("The solution of part one is {:?}", solution_1);

    let solution_2 = solve_part_two();

    println!("The solution of part two is {:?}", solution_2);

    Ok(())
}

fn run_session() -> FileSystem {
    let file_system = FileSystem {
        root: Rc::new(RefCell::new(FileSystemNode {
            parent: None,
            dirs: HashMap::new(),
            files: vec![],
        })),
    };
    let _run_session = include_str!("./day7.input")
        .lines()
        .map(|line| line.parse::<ShellLine>().unwrap())
        .fold(
            Session {
                current_dir: Rc::clone(&file_system.root),
            },
            |session, shell_line| match shell_line {
                ShellLine::OutputLine(File::Directory { .. }) => session,
                ShellLine::OutputLine(file) => session.add_file(file),
                ShellLine::InputLine(Command::CdUp) => session.cd_up(),
                ShellLine::InputLine(Command::CdDown(dir)) => session.cd_down(&dir.to_owned()),
                ShellLine::InputLine(Command::Ls) => session,
            },
        );

    file_system
}

fn solve_part_one() -> Result<usize> {
    let file_system = run_session();
    Ok(file_system
        .report()
        .iter()
        .filter(|(_, size)| *size < 100_000)
        .fold(0, |acc, (_, size)| acc + size))
}

fn solve_part_two() -> Result<usize> {
    let report = run_session().report();
    let used_space = report.last().unwrap().1;
    let needed_space = used_space - (70_000_000 - 30_000_000);
    Ok(report.iter().fold(used_space, |acc, (_, size)| {
        if *size > needed_space && *size < acc {
            *size
        } else {
            acc
        }
    }))
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn command_parse() {
        let command = "$ cd ..".parse::<Command>();
        assert_eq!(command.unwrap(), Command::CdUp);
        let command = "$ cd a".parse::<Command>();
        assert_eq!(command.unwrap(), Command::CdDown("a".to_string()));
        let command = "$ ls".parse::<Command>();
        assert_eq!(command.unwrap(), Command::Ls);
    }

    #[test]
    fn file_parse() {
        let command = "4060174 j".parse::<File>();
        assert_eq!(
            command.unwrap(),
            File::RegularFile {
                size: 4060174,
                name: "j".to_string()
            }
        );
        let command = "dir a".parse::<File>();
        assert_eq!(
            command.unwrap(),
            File::Directory {
                name: "a".to_string()
            }
        );
        let command = "8033020 d.log".parse::<File>();
        assert_eq!(
            command.unwrap(),
            File::RegularFile {
                size: 8033020,
                name: "d.log".to_string()
            }
        );
        let command = "dir d".parse::<File>();
        assert_eq!(
            command.unwrap(),
            File::Directory {
                name: "d".to_string()
            }
        );
        let command = "5626152 d.ext".parse::<File>();
        assert_eq!(
            command.unwrap(),
            File::RegularFile {
                size: 5626152,
                name: "d.ext".to_string()
            }
        );
    }

    #[test]
    fn line_parse() {
        let command = "$ cd ..".parse::<ShellLine>();
        assert_eq!(command.unwrap(), ShellLine::InputLine(Command::CdUp));
        let command = "$ cd a".parse::<ShellLine>();
        assert_eq!(
            command.unwrap(),
            ShellLine::InputLine(Command::CdDown("a".to_string()))
        );
        let command = "$ ls".parse::<ShellLine>();
        assert_eq!(command.unwrap(), ShellLine::InputLine(Command::Ls));
        let command = "dir d".parse::<ShellLine>();
        assert_eq!(
            command.unwrap(),
            ShellLine::OutputLine(File::Directory {
                name: "d".to_string()
            })
        );
        let command = "dir a".parse::<ShellLine>();
        assert_eq!(
            command.unwrap(),
            ShellLine::OutputLine(File::Directory {
                name: "a".to_string()
            })
        );
        let command = "4060174 j".parse::<ShellLine>();
        assert_eq!(
            command.unwrap(),
            ShellLine::OutputLine(File::RegularFile {
                size: 4060174,
                name: "j".to_string()
            })
        );
        let command = "8033020 d.log".parse::<ShellLine>();
        assert_eq!(
            command.unwrap(),
            ShellLine::OutputLine(File::RegularFile {
                size: 8033020,
                name: "d.log".to_string()
            })
        );
        let command = "5626152 d.ext".parse::<ShellLine>();
        assert_eq!(
            command.unwrap(),
            ShellLine::OutputLine(File::RegularFile {
                size: 5626152,
                name: "d.ext".to_string()
            })
        );
    }
}
