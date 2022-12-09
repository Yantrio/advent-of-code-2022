use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::iter::Map;
use std::rc::{Rc, Weak};
use std::str::FromStr;

fn main() {
    let input = include_str!("input");

    let commands: Vec<CommandExecution> = input
        .split("\n$ ")
        .map(|s| s.parse().unwrap())
        .collect::<Vec<_>>();

    let mut fs = FileSystem::new();
    fs.parse_command_executions(commands);

    println!(
        "Part 1: {:#?}",
        fs.get_all_flattened_directories()
            .iter()
            .filter(|&d| d.get_size() < 100000)
            .map(|d| d.get_size())
            .sum::<usize>()
    );

    println!("Part 2: {:#?}", part2(&fs))
}

fn part2(fs: &FileSystem) -> usize {
    let mut dirs = fs.get_all_flattened_directories();
    let disk_space_needed = 30000000 - (70000000 - dirs[0].get_size());

    dirs.iter()
        .map(|d| (d, d.get_size()))
        .filter(|&d| d.1 > disk_space_needed)
        .min_by(|a, b| a.1.cmp(&b.1))
        .unwrap()
        .0
        .get_size()
}

struct Directory {
    name: String,
    files: Vec<File>,
    directories: HashMap<String, Directory>,
}

impl Directory {
    fn new(name: String) -> Directory {
        Directory {
            name,
            files: vec![],
            directories: HashMap::new(),
        }
    }

    fn get_size(&self) -> usize {
        self.files.iter().map(|f| f.size).sum::<usize>()
            + self
                .directories
                .iter()
                .map(|(_, d)| d.get_size())
                .sum::<usize>()
    }

    fn get_all_directories(&self) -> Vec<&Directory> {
        let mut dirs = vec![self];
        for (_, d) in self.directories.iter() {
            dirs.extend(d.get_all_directories());
        }
        dirs
    }
}

impl fmt::Debug for Directory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Directory {{ name: {}, total_size: {:?} files: {:#?}, directories: {:#?} }}",
            self.name,
            self.get_size(),
            self.files,
            self.directories
        )
    }
}

#[derive(Debug)]
struct FileSystem {
    Directories: HashMap<String, Directory>,
    CurrentDirStack: Vec<String>,
}

impl FileSystem {
    fn new() -> FileSystem {
        let mut fs = FileSystem {
            Directories: HashMap::new(),
            CurrentDirStack: vec![],
        };

        fs.Directories
            .insert("/".to_string(), Directory::new("/".to_string()));

        fs
    }

    fn get_current_directory(&mut self) -> &mut Directory {
        let mut cwd = self
            .Directories
            .get_mut(self.CurrentDirStack.first().unwrap())
            .unwrap();
        for p in self.CurrentDirStack.iter().skip(1) {
            cwd = cwd.directories.get_mut(p).unwrap();
        }
        cwd
    }

    fn get_all_flattened_directories(&self) -> Vec<&Directory> {
        let mut dirs = vec![];
        for (_, d) in self.Directories.iter() {
            dirs.extend(d.get_all_directories());
        }
        dirs
    }

    fn parse_command_executions(&mut self, command_executions: Vec<CommandExecution>) {
        for c in command_executions.into_iter() {
            match c.command.binary {
                Binary::ChangeDirectory(s) => match s.as_str() {
                    ".." => {
                        self.CurrentDirStack.pop();
                    }
                    "." => {}
                    _ => {
                        self.CurrentDirStack.push(s);
                    }
                },
                Binary::List => {
                    let cwd = self.get_current_directory();
                    for line in c.stdout {
                        let split = line.split(" ").collect::<Vec<_>>();
                        match split[0] {
                            "dir" => {
                                cwd.directories.insert(
                                    split[1].to_string(),
                                    Directory::new(split[1].to_string()),
                                );
                            }
                            a => {
                                cwd.files.push(line.parse().unwrap());
                            }
                            _ => panic!("Unknown file type"),
                        }
                    }
                }

                _ => {}
            }
        }
    }
}

#[derive(Debug)]
struct File {
    name: String,
    size: usize,
}

impl FromStr for File {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(" ");
        let size = parts.next().unwrap().parse().unwrap();
        let name = parts.next().unwrap().to_string();
        Ok(File { name, size })
    }
}

#[derive(Debug)]
enum Binary {
    Unknown,

    ChangeDirectory(String),
    List,
}

#[derive(Debug)]
struct Command {
    binary: Binary,
    arguments: Vec<String>,
}

impl FromStr for Command {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();
        let command = parts.next().unwrap();
        let arguments = parts.map(|s| s.to_string()).collect::<Vec<_>>();

        let binary = match command {
            "cd" => Binary::ChangeDirectory(arguments[0].clone()),
            "ls" => Binary::List,
            _ => Binary::Unknown,
        };

        Ok(Command { binary, arguments })
    }
}

#[derive(Debug)]
struct CommandExecution {
    command: Command,
    stdout: Vec<String>,
}

impl FromStr for CommandExecution {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let trimmed = s.trim();
        Ok(CommandExecution {
            command: trimmed
                .lines()
                .next()
                .unwrap()
                .replace("$ ", "")
                .parse()
                .unwrap(),
            stdout: trimmed
                .lines()
                .skip(1)
                .map(|s| s.to_owned())
                .collect::<Vec<String>>(),
        })
    }
}
