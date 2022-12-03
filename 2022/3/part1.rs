// Advent of Code Day 3 Part 1

use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn prioritize_item(item: &char) -> u32 {
    if *item >= 'a' && *item <= 'z' {
        // a-z have priorities 1-26
        (*item as u8 - b'a' + 1) as u32
    } else if *item >= 'A' && *item <= 'Z' {
        // A-Z have priorities 27-52
        (*item as u8 - b'A' + 27) as u32
    } else {
        panic!("Invalid item in input: {}", item)
    }
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut priority_sum: u32 = 0;
    for line in reader.lines() {
        let line_str = line.unwrap();

        // Ignore empty lines
        if line_str.len() == 0 {
            continue;
        }

        // Each rucksack will be divided into two equal sized compartments
        assert!(line_str.len() % 2 == 0); // no handling for odd lines, just err
        let mid = line_str.len() / 2;
        let (comp1, comp2) = (&line_str[0..mid], &line_str[mid..]);
        let comp1_items: HashSet<char> = comp1.chars().collect();
        let comp2_items: HashSet<char> = comp2.chars().collect();
        let common_items = comp1_items.intersection(&comp2_items);

        priority_sum += common_items.map(prioritize_item).sum::<u32>();
    }

    println!("Sum of common compartment priorities: {}", priority_sum);
}
