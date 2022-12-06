use nom::{
    branch::alt,
    bytes::complete::{is_a, tag},
    character::complete::{char, digit0, satisfy, space1},
    combinator::{map, map_res},
    multi::{separated_list0, separated_list1},
    sequence::{delimited, tuple},
    IResult,
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
    }

    dock_1
        .iter()
        .map(|x| x[x.len() - 1])
        .for_each(|x| println!("{x}"));

    println!("2nd");

    let mut dock_2 = dock;

    for x in instructions {
        x.execute_2(&mut dock_2);
    }

    dock_2
        .iter()
        .map(|x| x[x.len() - 1])
        .for_each(|x| println!("{x}"));
}

fn initial_state(input: &str) -> IResult<&str, Vec<Vec<char>>> {
    let (rest, mut crates) = crate_space(input)?;
    let (rest, labels) = label_list(rest)?;

    let mut vec = Vec::with_capacity(labels.len());

    labels.iter().rev().copied().for_each(|label| {
        let mut stack = Vec::new();

        for x in crates.iter_mut().rev() {
            if let Some(y) = x[label as usize - 1] {
                stack.push(y)
            }
        }

        vec.push(stack);
    });

    vec.reverse();

    Ok((rest, vec))
}

fn instruction_all(input: &str) -> IResult<&str, Vec<Instruction>> {
    separated_list0(char('\n'), instruction)(input)
}

fn instruction(input: &str) -> IResult<&str, Instruction> {
    let (rest, (_, _, amount, _, _, _, from, _, _, _, to)) = tuple((
        is_a("move"),
        space1,
        u8digit,
        space1,
        is_a("from"),
        space1,
        u8digit,
        space1,
        is_a("to"),
        space1,
        u8digit,
    ))(input)?;

    Ok((rest, Instruction { amount, from, to }))
}

fn crate_space(input: &str) -> IResult<&str, Vec<Vec<Option<char>>>> {
    let (rest, mut vec) = separated_list0(char('\n'), crate_line)(input)?;
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
        map(
            delimited(char('['), satisfy(|x| x.is_ascii_alphabetic()), char(']')),
            Some,
        ),
        map(tag("   "), |_| None),
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
    pub fn execute_1(&self, dock: &mut [Vec<char>]) {
        let mut to = dock.get(self.to as usize - 1).unwrap().clone();

        let from = dock.get_mut(self.from as usize - 1).unwrap();

        let drained = from.drain(from.len() - self.amount as usize..);

        to.extend(drained.rev());

        dock[self.to as usize - 1] = to;
    }

    pub fn execute_2(&self, dock: &mut [Vec<char>]) {
        let mut to = dock.get(self.to as usize - 1).unwrap().clone();

        let from = dock.get_mut(self.from as usize - 1).unwrap();

        let drained = from.drain(from.len() - self.amount as usize..);

        to.extend(drained);

        dock[self.to as usize - 1] = to;
    }
}
