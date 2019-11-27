use std::io;
use std::io::prelude::*;
use std::collections::VecDeque;
use std::collections::HashSet;

fn reactable(p1: char, p2: char) -> bool {
    p1 != p2 &&  p1.to_ascii_lowercase() == p2.to_ascii_lowercase()
}

fn react<'a>(polymer: impl Iterator<Item = &'a char>) -> Vec<char> {
    let mut finished: VecDeque<char> = VecDeque::new();
    for p in polymer {
        if reactable(*finished.back().unwrap_or(&'_'), *p) {
            finished.pop_back();
        } else {
            finished.push_back(*p);
        }
    }
    finished.into()
}

fn main() {
    let stdin = io::stdin();
    let mut stdin = stdin.lock().lines();
    let input: Vec<char> = stdin.next().unwrap().unwrap().chars().collect();
    println!("Part 1: {}", react(input.iter()).len());

    let alphabet = input.iter().map(|c| c.to_ascii_lowercase()).collect::<HashSet<char>>();
    let mut part2 = std::usize::MAX;
    for &letter in &alphabet {
        let reduced_input = input.iter().filter(|c| c.to_ascii_lowercase() != letter);
        let len = react(reduced_input).len();
        part2 = if len < part2 { len } else { part2 };
    }
    println!("Part 2: {}", part2);
}
