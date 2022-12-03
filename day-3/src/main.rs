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

    let solution_2: u32 = lines
        .chunks(3)
        .map(find_badge)
        .map(priority)
        .sum();

    println!("{solution_2}");
}

fn priority(c: char) -> u32 {
    if c.is_ascii_uppercase() {
        c as u32 - 65 + 27
    } else {
        c as u32 - 96
    }
}

fn find_badge(strings: &[&str]) -> char {
    let mut in_all_3 = strings[0]
        .chars()
        .filter(|c| strings[1..].iter().all(|s| s.contains(*c)));

    in_all_3.next().unwrap()
}
