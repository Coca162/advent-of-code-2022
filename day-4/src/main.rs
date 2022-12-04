const INPUT: &str = include_str!("input.txt");

fn main() {
    let elf_tuple_pairs: Vec<((u8, u8), (u8, u8))> = INPUT
        .lines()
        .map(|elves| elves.split_once(',').unwrap())
        .map(|(first, second)| {
            (
                first.split_once('-').unwrap(),
                second.split_once('-').unwrap(),
            )
        })
        .map(|((first_start, first_end), (second_start, second_end))| {
            (
                (
                    first_start.parse().unwrap(),
                    first_end.parse().unwrap(),
                ),
                (
                    second_start.parse().unwrap(),
                    second_end.parse().unwrap(),
                ),
            )
        })
        .collect();

    let inside = elf_tuple_pairs
        .iter()
        .filter(|((first_min, first_max), (second_min, second_max))|
            (first_min >= second_min && first_max <= second_max)
                || (second_min >= first_min && second_max <= first_max)
        )
        .count();

    let overlaps = elf_tuple_pairs
        .iter()
        .filter(|((first_min, first_max), (second_min, second_max))|
            first_max >= second_min && second_max >= first_min
        )
        .count();

    println!("{inside}");

    println!("{overlaps}")
}
