use std::io;
use std::str::FromStr;
use std::io::prelude::*;
use std::collections::HashMap;

#[derive(Debug)]
enum Event {
    NewGuard(u16),
    Wake,
    FallAsleep
}

#[derive(Debug)]
struct Note {
    minute: u8,
    event: Event
}

fn main() {
    let stdin = io::stdin();
    let stdin = stdin.lock().lines();
    let mut input: Vec<String> = stdin.map(|n| n.unwrap()).collect();
    input.sort();

    type Record = [u16; 60];
    let mut records = HashMap::<u16, Record>::new();

    let mut currentguard = 0;
    let mut sleepstart = 0;

    for line in &input {
        let note: Note = line.parse().unwrap();
        match note.event {
            Event::NewGuard(id) => currentguard = id,
            Event::Wake => {
                let record = records.entry(currentguard).or_insert([0; 60]);
                for i in sleepstart..note.minute {
                    record[i as usize] += 1;
                }
            }
            Event::FallAsleep => sleepstart = note.minute
        }
    }

    let (&guardid, record) = records.iter().max_by_key(|(_, record)| record.iter().sum::<u16>()).unwrap();
    let (minute, _) = record.iter().enumerate().max_by_key(|(_, sleep)| *sleep).unwrap();
    println!("Part 1: {} * {} = {}", guardid, minute, guardid as usize * minute);

    let (&guardid, record) = records.iter().max_by_key(|(_, record)| record.iter().max()).unwrap();
    let (minute, _) = record.iter().enumerate().max_by_key(|(_, sleep)| *sleep).unwrap();
    println!("Part 2: {} * {} = {}", guardid, minute, guardid as usize * minute);
}

impl FromStr for Note {
    type Err = &'static str;
    fn from_str(line: &str) -> Result<Note, Self::Err> {
        let line: Vec<&str> = line.split_ascii_whitespace().collect();
        let minute = |time: &str| time.chars().filter(|c| c.is_numeric()).skip(2).collect::<String>().parse::<u8>().unwrap();
        match line.as_slice() {
            [_, time, _, id, _, _] => {
                let id = id.chars().filter(|c| c.is_numeric()).collect::<String>().parse().unwrap();
                Ok(Note { minute: minute(time),  event: Event::NewGuard(id)})
            },
            [_, time, action, _] => {
                let event = if *action == "falls" { Event::FallAsleep } else { Event::Wake };
                Ok(Note { minute: minute(time),  event})
            }
            _ => Err("line not recognized")
        }
    }
}