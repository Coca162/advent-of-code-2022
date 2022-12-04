const INPUT: &str = include_str!("input.txt");

fn main() {
    let elf_tuple_pairs = INPUT
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
                    first_start.parse::<u32>().unwrap(),
                    first_end.parse::<u32>().unwrap(),
                ),
                (
                    second_start.parse::<u32>().unwrap(),
                    second_end.parse::<u32>().unwrap(),
                ),
            )
        })
        .collect::<Vec<_>>();

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
