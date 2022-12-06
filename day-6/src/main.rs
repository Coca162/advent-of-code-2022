#![feature(iter_next_chunk)]

use std::collections::VecDeque;

const INPUT: &str = include_str!("input.txt");

fn main() {
    let answer1 = solution::<4>().unwrap();

    let answer2 = solution::<14>().unwrap();

    println!("{answer1}");

    println!("{answer2}");
}

fn solution<const N: usize>() -> Option<usize> {
    //Pretend this is a streamed input
    let mut bytes = INPUT.bytes();

    let mut latest = bytes.next_chunk::<N>().unwrap();

    latest.reverse();

    if no_dup(&latest) {
        return Some(N);
    }

    let mut latest = VecDeque::from(latest);

    for (distance, byte) in bytes.enumerate() {
        if no_dup(latest.make_contiguous()) {
            return Some(distance + N);
        }

        latest.pop_back();
        latest.push_front(byte);
    }

    None
}

fn no_dup(array: &[u8]) -> bool {
    !(1..array.len()).any(|i| array[i..].contains(&array[i - 1]))
}
