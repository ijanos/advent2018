use std::io;
use std::io::prelude::*;
use std::collections::HashMap;

fn main() {
    let stdin = io::stdin();
    let stdin = stdin.lock().lines();
    let mut input = Vec::<(u16, u16, u16, u16, u16)>::new();
    for line in stdin {
        let line = line.unwrap().replace(|x: char| !x.is_numeric(), " ");
        let numbers: Vec<u16> = line.split_whitespace().map(|num| num.parse().unwrap()).collect();
        input.push((numbers[0], numbers[1], numbers[2], numbers[3], numbers[4]));
    }
    let mut fabric = HashMap::<(u16, u16), u16>::new();
    for &(_id, x, y, w, h) in &input {
        for a in x..x+w {
            for b in y..y+h {
                let counter = fabric.entry((a, b)).or_insert(0);
                *counter += 1;
            }
        }
    }
    let part1 = fabric.values().filter(|&&n| n > 1).count();
    println!("Part 1: {}", part1);
    for &(id, x, y, w, h) in &input {
        let mut part2 = true;
        for a in x..x+w {
            for b in y..y+h {
                if *fabric.get(&(a, b)).unwrap() > 1 {
                    part2 = false;
                    break;
                }
            }
        }
        if part2 {
            println!("Part 2: {}", id);
        }
    }
}