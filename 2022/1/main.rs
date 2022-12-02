// Advent of Code Day 1

use std::collections::BinaryHeap;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
pub struct ElfBackpack {
    pub calories: Vec<u64>,
}

#[derive(Debug)]
pub struct ElfLedger {
    pub elves: Vec<ElfBackpack>,
}

impl ElfLedger {
    pub fn from_file(file_path: &str) -> Result<ElfLedger, Box<dyn Error>> {
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);
        let mut ledger: ElfLedger = ElfLedger { elves: Vec::new() };
        let mut current_elf: ElfBackpack = ElfBackpack {
            calories: Vec::new(),
        };

        for line in reader.lines() {
            let value = line?;
            if value == "" {
                ledger.elves.push(current_elf);
                current_elf = ElfBackpack {
                    calories: Vec::new(),
                };
            } else {
                current_elf.calories.push(value.parse().unwrap());
            }
        }

        // File might not end with a newline
        if current_elf.calories.len() > 0 {
            ledger.elves.push(current_elf);
        }

        Ok(ledger)
    }
}

fn main() {
    let ledger = ElfLedger::from_file("input.txt").expect("Error parsing file");

    // BinaryHeap is by default a max heap
    let mut cals: BinaryHeap<u64> = BinaryHeap::new();
    for elf in ledger.elves {
        cals.push(elf.calories.iter().sum());
    }

    println!("Max cals: {}", cals.peek().unwrap());
    let top3: u64 = cals.pop().unwrap() + cals.pop().unwrap() + cals.pop().unwrap();
    println!("Sum of top 3 cals: {}", top3);
}
