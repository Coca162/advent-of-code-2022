use std::collections::HashMap;
use nom::{
    IResult,
    bytes::complete::{tag, is_a},
    character::complete::{satisfy, char, digit0, space1},
    combinator::{map, map_res},
    sequence::{delimited, tuple},
    multi::{separated_list0, separated_list1},
    branch::alt
};

const INPUT: &str = include_str!("input.txt");

fn main() {
    let input = INPUT.replace("\r\n", "\n");

    let (rest, dock) = initial_state(&input).unwrap();

    let (_, instructions) = instruction_all(rest.trim()).unwrap();

    println!("1st");

    let mut dock_1 = dock.clone();

    for x in &instructions {
        x.execute_1(&mut dock_1);
    };

    let mut day_1 = dock_1.iter().collect::<Vec<_>>();

    day_1.sort_by_key(|x| x.0);

    day_1.iter().map(|x| x.1[x.1.len() - 1]).for_each(|x| println!("{x}"));

    println!("2nd");

    let mut dock_2 = dock.clone();

    for x in instructions {
        x.execute_2(&mut dock_2);
    };

    let mut day_2 = dock_2.iter().collect::<Vec<_>>();

    day_2.sort_by_key(|x| x.0);

    day_2.iter().map(|x| x.1[x.1.len() - 1]).for_each(|x| println!("{x}"));

}

fn initial_state(input: &str) -> IResult<&str, HashMap<u8, Vec<char>>> {
    let (rest, mut crates) = crate_space(input)?;
    let (rest, labels) = label_list(rest)?;

    println!("{crates:?}");

    let mut hash = HashMap::with_capacity(labels.len());

    labels.iter().rev().copied().for_each(|label| {
        let mut stack= Vec::new();

        for x in crates.iter_mut().rev() {
            if let Some(y) = x[label as usize - 1] { 
                stack.push(y) 
            }
        }

        hash.insert(label, stack);
    });

    Ok((rest, hash))
}

fn instruction_all(input: &str) -> IResult<&str, Vec<Instruction>> {
    separated_list0(char('\n'), instruction)(input)
}

fn instruction(input: &str) -> IResult<&str, Instruction> {
    let (rest, (_ , _ , amount , _ , _ , _ , from , _ , _ , _ , to)) = 
    tuple((is_a("move") , space1, u8digit, space1, is_a("from"), space1, u8digit, space1, is_a("to"), space1, u8digit))(input)?;

    Ok((rest, Instruction { amount, from, to }))
}

fn crate_space(input: &str) -> IResult<&str, Vec<Vec<Option<char>>>> {
    let (rest,mut vec) = separated_list0(char('\n'), crate_line)(input)?;
    vec.pop();
    Ok((rest, vec))
}

fn label_list(input: &str) -> IResult<&str, Vec<u8>> {
    separated_list1(char(' '), label)(input)
}

fn crate_line(input: &str) -> IResult<&str, Vec<Option<char>>> {
    separated_list0(char(' '), crate_box)(input)
}

fn crate_box(input: &str) -> IResult<&str, Option<char>> {
    alt((
        map(delimited(char('['), satisfy(|x| x.is_ascii_alphabetic()), char(']')), Some),
        map(tag("   "), |_| None)
    ))(input)
}

fn label(input: &str) -> IResult<&str, u8> {
    delimited(char(' '), u8digit, char(' '))(input)
}

fn u8digit(input: &str) -> IResult<&str, u8> {
    map_res(digit0, |s: &str| s.parse())(input)
}

struct Instruction {
    amount: u8,
    from: u8,
    to: u8,
}

impl Instruction {
    pub fn execute_1(&self, dock: &mut HashMap<u8, Vec<char>>) {
        let mut to = dock.get(&self.to).unwrap().clone();

        let from = dock.get_mut(&self.from).unwrap();

        let drained = from.drain(from.len() - self.amount as usize..);

        to.extend(drained);

        dock.insert(self.to, to);
    }

    pub fn execute_2(&self, dock: &mut HashMap<u8, Vec<char>>) {
        let mut to = dock.get(&self.to).unwrap().clone();

        let from = dock.get_mut(&self.from).unwrap();

        let drained = from.drain(from.len() - self.amount as usize..);

        to.extend(drained.rev());

        dock.insert(self.to, to);
    }
}