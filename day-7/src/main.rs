use std::collections::HashMap;

use nom::{
    branch::alt,
    bytes::complete::{tag, is_not},
    character::complete::{char, digit0, space1},
    combinator::{map, map_res},
    multi::{separated_list0, many0},
    sequence::tuple,
    IResult,
};

const INPUT: &str = include_str!("input.txt");

fn main() {
    let (_, (star_1, star_2)) = solution(&INPUT.replace("\r\n", "\n")).unwrap();

    println!("{star_1} {star_2}");
}

#[derive(Debug)]
struct Directory {
    children: HashMap<String, Box<Directory>>,
    files: Vec<File>
}

#[derive(Debug)]
struct File {
    #[allow(dead_code)]
    name: String,
    size: u64
}

impl Directory {
    fn new() -> Directory {
        Directory{children: Default::default(), files: Default::default()}
    }

    fn size(&self) -> u64 {
        let file_size: u64 = self.files.iter().map(|x| x.size).sum();
        let child_size: u64 = self.children.iter().map(|x| x.1.size()).sum();
        child_size + file_size
    }

    fn all_dirs(&self, root: bool) -> Vec<u64> {
        let size = self.size();
        let mut dirs: Vec<u64> = self.children.iter().flat_map(|x| x.1.all_dirs(false)).collect();

        if !root {
            dirs.push(size);
        }

        dirs
    }
}

enum Command {
    ChangeDirectory(DirectoryType),
    List((Vec<String>, Vec<File>))
}

enum Item {
    Directory(String),
    File(File)
}

enum DirectoryType {
    Root,
    Parent,
    Child(String)
}

fn solution(input: &str)-> IResult<&str, (u64, u64)> {
    let mut root = Box::new(Directory::new());

    let mut current_path: Vec<String> = Vec::new();

    let mut current = &mut root;

    let (rest, commands) = commands(input)?;

    for command in  commands {
        match command {
            Command::ChangeDirectory(cd) => {
                match cd {
                    DirectoryType::Root => (),
                    DirectoryType::Parent => { 
                        current_path.pop().unwrap();
                        let mut iter = current_path.iter();

                        let not_root = iter.next().and_then(|x| root.children.get_mut(x));
                        match not_root {
                            Some(x) => {
                                current = x;
                                for dir in iter {
                                    current = current.children.get_mut(dir).unwrap();
                                }
                            },
                            None => current = &mut root,
                        }

                    },
                    DirectoryType::Child(name) => {
                        current = current.children.get_mut(&name).unwrap();
                        current_path.push(name);
                    }
                }
            },
            Command::List((dirs, files)) => {
                for ele in dirs {
                    current.children.insert(ele, Box::new(Directory::new()));
                }

                current.files = files;
            },
        }
    }

    let mut all: Vec<u64> = root.all_dirs(true);

    all.sort();

    let free_up = 30000000 - (70000000 - root.size());

    Ok((rest, (all.iter().filter(|x| x <= &&100000).sum(), *all.iter().find(|x| x >= &&free_up).unwrap())))
}

fn commands(input: &str) -> IResult<&str, Vec<Command>> {
    many0(map_command)(input)
}

fn map_command(input: &str) -> IResult<&str, Command> {
    alt((map(change_directory, Command::ChangeDirectory), map(list, Command::List)))(input)
}

fn change_directory(input: &str) -> IResult<&str, DirectoryType> {
    let (rest, (_,_,_, _, name, _)) = tuple((char('$'), space1, tag("cd"), space1, is_not("\n"), char('\n')))(input)?;

    Ok((rest, 
    match name {
        "/" => DirectoryType::Root,
        ".." => DirectoryType::Parent,
        name => DirectoryType::Child(name.to_string())
    }))
}

fn list(input: &str) -> IResult<&str, (Vec<String>, Vec<File>)> {
    let (rest, (_,_,_, _, items)) = tuple((char('$'), space1, tag("ls"), char('\n'), is_not("$")))(input)?;

    Ok((rest, map_items(items.trim_end_matches('\n'))?.1))
}

fn map_items(input: &str) -> IResult<&str, (Vec<String>, Vec<File>)> {
    let mut dirs = Vec::new();
    let mut files = Vec::new();
    
    let (rest, items) = separated_list0(char('\n'), map_item)(input)?;

    for item in items {
        match item {
            Item::Directory(x) => dirs.push(x),
            Item::File(x) => files.push(x),
        }
    }

    Ok((rest, (dirs, files)))
}

fn map_item(input: &str) -> IResult<&str, Item> {
    alt((map(map_file, Item::File), map(map_directory, Item::Directory)))(input)
}

fn map_directory(input: &str) -> IResult<&str, String> {
    let (rest, (_, _, name)) = tuple((tag("dir"), space1, is_not("\n")))(input)?;
    Ok((rest, name.to_string()))
}

fn map_file(input: &str) -> IResult<&str, File> {
    let (rest, (size, _, name)) = tuple((u64digit, space1, is_not("\n")))(input)?;
    Ok((rest, File{ name: name.to_string(), size }))
}

fn u64digit(input: &str) -> IResult<&str, u64> {
    map_res(digit0, |s: &str| s.parse())(input)
}