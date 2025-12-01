use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    Left,
    Right,
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut index: i64 = 50;
    let mut zero_count: u64 = 0;

    for line_result in reader.lines() {
        let line = line_result.unwrap();
        let (dir_str, count_str) = line.split_at(1);
        let direction = match dir_str {
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => panic!("Invalid direction"),
        };
        let count = count_str.parse::<i64>().unwrap();

        match direction {
            Direction::Left => {
                index -= count;
            }
            Direction::Right => {
                index += count;
            }
        };

        while index < 0 {
            index += 100;
        }

        if index % 100 == 0 {
            zero_count += 1;
        }
    }

    println!("Zero count: {}", zero_count)
}
