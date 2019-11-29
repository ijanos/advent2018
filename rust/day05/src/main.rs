use std::io;
use std::io::prelude::*;
use std::collections::HashSet;

fn reactable(p1: char, p2: char) -> bool {
    p1 != p2 &&  p1.to_ascii_lowercase() == p2.to_ascii_lowercase()
}

fn react<'a>(polymer: impl Iterator<Item = &'a char>) -> usize {
    let mut finished: Vec<char> = Vec::new();
    for p1 in polymer {
        match finished.last() {
            Some(p2) if reactable(*p1, *p2) => { finished.pop(); },
            _ => { finished.push(*p1); }
        }
    }
    finished.len()
}

fn main() {
    let stdin = io::stdin();
    let mut stdin = stdin.lock().lines();
    let input: Vec<char> = stdin.next().unwrap().unwrap().chars().collect();
    println!("Part 1: {}", react(input.iter()));
    let alphabet = input.iter().map(|c| c.to_ascii_lowercase()).collect::<HashSet<char>>();
    let part2 = alphabet.iter().map(|&letter| react(input.iter().filter(|c| c.to_ascii_lowercase() != letter))).min().unwrap();
    println!("Part 2: {}", part2);
}
