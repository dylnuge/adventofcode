use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("input.txt").expect("Failed to open file");
    let reader = BufReader::new(file);

    let mut sum = 0;

    for line in reader.lines() {
        let rucksack = line.expect("Failed to read line");

        let (comp1, comp2) = rucksack.split_at(rucksack.len() / 2);
        let compartments: Vec<&str> = vec![comp1, comp2];

        let mut counts = [0; 52];
        for c in compartments[0].chars() {
            if c.is_ascii_lowercase() {
                counts[(c as u8 - b'a') as usize] += 1;
            } else if c.is_ascii_uppercase() {
                counts[(c as u8 - b'A') as usize + 26] += 1;
            }
        }

        // Use a hash set to keep track of items that have already been counted
        let mut counted = HashSet::new();
        for c in compartments[1].chars() {
            if c.is_ascii_lowercase() {
                let i = (c as u8 - b'a') as usize;
                if counts[i] > 0 && !counted.contains(&i) {
                    sum += i + 1;
                    counted.insert(i);
                }
            } else if c.is_ascii_uppercase() {
                let i = (c as u8 - b'A') as usize + 26;
                if counts[i] > 0 && !counted.contains(&i) {
                    sum += i + 1;
                    counted.insert(i);
                }
            }
        }
    }

    println!("Sum of priorities: {}", sum);
}
