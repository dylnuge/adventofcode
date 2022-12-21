// Advent of Code 2022 Day 21 Part 1

use std::collections::HashMap;
use std::fs;

fn process_monkey(name: &str, monkeys: &HashMap<String, String>) -> isize {
    let instruction = monkeys.get(name).unwrap();
    if let Ok(num) = instruction.parse::<isize>() {
        return num;
    }

    let elems: Vec<&str> = instruction.split_whitespace().collect();
    assert!(elems.len() == 3);
    let left = process_monkey(elems[0], monkeys);
    let right = process_monkey(elems[2], monkeys);

    match elems[1] {
        "+" => left + right,
        "-" => left - right,
        "/" => left / right,
        "*" => left * right,
        _ => panic!("bad operation {}", elems[1]),
    }
}

fn main() {
    let monkey_data = fs::read_to_string("input.txt").unwrap();
    let mut monkeys: HashMap<String, String> = HashMap::new();

    for line in monkey_data.lines() {
        let mut elems = line.split(": ");
        let monkey_name = elems.next().unwrap();
        let instruction = elems.next().unwrap();
        monkeys.insert(monkey_name.to_owned(), instruction.to_owned());
    }

    println!("{:?}", process_monkey("root", &monkeys));
}
