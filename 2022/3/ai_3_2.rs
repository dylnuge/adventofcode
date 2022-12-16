use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("input.txt").expect("Failed to open file");
    let reader = BufReader::new(file);

    let mut sum = 0;

    // Keep track of the rucksacks in the current group
    let mut group = Vec::new();

    for line in reader.lines() {
        let rucksack = line.expect("Failed to read line");

        // Add the current rucksack to the group
        group.push(rucksack);

        // If we have three rucksacks in the group, process them
        if group.len() == 3 {
            // Keep track of which items appear in which rucksacks
            let mut rucksacks = [false; 3];
            // Keep track of which items appear in the group
            let mut counts = [false; 52];

            for i in 0..3 {
                for c in group[i].chars() {
                    if c.is_ascii_lowercase() {
                        let j = (c as u8 - b'a') as usize;
                        rucksacks[i] = true;
                        counts[j] = true;
                    } else if c.is_ascii_uppercase() {
                        let j = (c as u8 - b'A') as usize + 26;
                        rucksacks[i] = true;
                        counts[j] = true;
                    }
                }
            }

            // Check if any items appear in all three rucksacks and no others
            for i in 0..52 {
                if rucksacks[0]
                    && rucksacks[1]
                    && rucksacks[2]
                    && counts[i]
                    && !rucksacks[0..3].contains(&false)
                {
                    sum += i + 1;
                }
            }

            // Clear the group and counts arrays after processing them
            group.clear();
            rucksacks = [false; 3];
            counts = [false; 52];
        }
    }

    println!("Sum of priorities: {}", sum);
}
