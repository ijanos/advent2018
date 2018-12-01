use std::io;
use std::io::prelude::*;
use std::collections::HashSet;

fn main() {
    let stdin = io::stdin();
    let stdin = stdin.lock().lines();
    let input: Vec<i32> = stdin.map(|n| n.unwrap().parse().unwrap()).collect();

    let part1: i32 = input.iter().sum();
    println!("part 1: {}", part1);

    let mut freq = 0;
    let mut seen = HashSet::new();
    for drift in input.into_iter().cycle() {
        freq += drift;
        if seen.contains(&freq) {
            break;
        } else {
            seen.insert(freq);
        }
    }
    println!("part 2: {}", freq)
}