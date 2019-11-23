use std::io;
use std::io::prelude::*;
use std::collections::HashMap;

fn main() {
    let stdin = io::stdin();
    let stdin = stdin.lock().lines();
    let input: Vec<String> = stdin.map(|n| n.unwrap()).collect();

    let counter = |line: &String| {
        let mut counts = HashMap::new();
        for ch in line.chars() {
            let counter = counts.entry(ch).or_insert(0);
            *counter += 1;
        }
        counts.values().cloned().collect::<Vec<i32>>()
    };

    let (twos, threes) = input.iter().fold((0, 0), |(twos, threes), line| {
        let c = counter(&line);
        (twos + if c.contains(&2) { 1 } else { 0 }, threes + if c.contains(&3) { 1 } else { 0 })
    });

    println!("Part 1: {}", twos * threes);

    let common_chars = |line1: &str, line2: &str| {
        line1.chars().zip(line2.chars()).filter(|(c1, c2)| c1 == c2).map(|(c1, _)| c1).collect::<String>()
    };

    for (i, line1) in input.iter().enumerate() {
        for line2 in &input[i..] {
            let common = common_chars(line1, line2);
            if common.len() == line1.len() - 1 {
                println!("Part 2: {}", common);
            }
        }
    }
}
