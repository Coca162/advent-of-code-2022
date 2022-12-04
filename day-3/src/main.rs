use core::panic;

const INPUT: &str = include_str!("input.txt");

fn main() {
    let lines = INPUT.lines().collect::<Vec<_>>();

    let solution_1: u32 = lines
        .iter()
        .map(|rucksack| rucksack.split_at(rucksack.len() / 2))
        .map(|(first, second)| first.chars().find(|c| second.contains(*c)).unwrap())
        .map(priority)
        .sum();

    println!("{solution_1}");

    let solution_2: u32 = lines.chunks(3).map(find_badge).map(priority).sum();

    println!("{solution_2}");
}

const SMALL_A_UNICODE: u32 = 97;
const CAPITAL_A_UNICODE: u32 = 65;
const LETTERS: u32 = 26;

fn priority(c: char) -> u32 {
    match c {
        'a'..='z' => c as u32 - SMALL_A_UNICODE + 1,
        'A'..='Z' => c as u32 - CAPITAL_A_UNICODE + LETTERS + 1,
        _ => panic!("Omg no way"),
    }
}

fn find_badge(strings: &[&str]) -> char {
    let mut in_all_3 = strings[0]
        .chars()
        .filter(|c| strings[1..].iter().all(|s| s.contains(*c)));

    in_all_3.next().unwrap()
}
