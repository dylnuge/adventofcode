// AoC 2022 Day 11

use std::collections::BinaryHeap;
use std::collections::VecDeque;
use std::fs;

#[derive(Debug)]
pub enum Operation {
    Add(usize),
    Multiply(usize),
    Square(),
    Double(),
}

#[derive(Debug)]
pub struct Monkey {
    // While not explicitly *stated*, the operations supported are addition and
    // multiplication and the starting values are all positive, so item values
    // should always be natural numbers (i.e. unsigned is fine)
    pub items: VecDeque<usize>,
    pub operation: Operation,
    // Again, the only test laid out in the spec is divisibility
    pub test_divisor: usize,
    // These are indexes for the monkey to pass to
    pub true_monkey: usize,
    pub false_monkey: usize,

    // This is a count of the number of times this monkey has investigated an
    // item
    pub inspection_count: usize,
}

fn get_monkeys_from_file(filename: &str) -> Vec<Monkey> {
    let mut monkeys: Vec<Monkey> = Vec::new();

    let monkey_data = fs::read_to_string(filename).unwrap();
    let mut lines = monkey_data.lines();

    loop {
        if let Some(line) = lines.next() {
            let monkey_data: Vec<&str> = line.split_whitespace().collect();
            // In the spec, monkeys are sequentially zero indexed, so we ignore
            // the index value given. If instead monkeys could be arbitrary
            // numbers, we would use a map instead of a vec.
            if monkey_data.len() >= 1 && monkey_data[0] == "Monkey" {
                // Parse items
                let item_line = lines.next().unwrap();
                // Item values start two characters past the colon on this line
                let num_start = item_line.find(":").unwrap() + 2;
                let items: VecDeque<usize> = item_line[num_start..]
                    .split(", ")
                    .map(|val| val.parse::<usize>().unwrap())
                    .collect();

                // Parse operation. Assume from spec that the operation line
                // always begins with "Operation: new = old" and ignore that
                // part, but we could handle or verify it if needed
                let op_line = lines.next().unwrap();
                let op_parts: Vec<&str> = op_line.split_whitespace().collect();
                let operation = if op_parts[5] == "old" {
                    match op_parts[4] {
                        "+" => Operation::Double(),
                        "*" => Operation::Square(),
                        _ => panic!("Unknown operation {}", op_parts[4]),
                    }
                } else {
                    let op_val = op_parts[5].parse::<usize>().unwrap();
                    match op_parts[4] {
                        "+" => Operation::Add(op_val),
                        "*" => Operation::Multiply(op_val),
                        _ => panic!("Unknown operation {}", op_parts[4]),
                    }
                };

                // Parse the numeric divisbility test, true throw, and false
                // throw lines. These lines are fixed format except for the
                // number at the very end.
                let div_line = lines.next().unwrap();
                let test_divisor = div_line
                    .split_whitespace()
                    .last()
                    .unwrap()
                    .parse::<usize>()
                    .unwrap();
                let true_line = lines.next().unwrap();
                let true_monkey = true_line
                    .split_whitespace()
                    .last()
                    .unwrap()
                    .parse::<usize>()
                    .unwrap();
                let false_line = lines.next().unwrap();
                let false_monkey = false_line
                    .split_whitespace()
                    .last()
                    .unwrap()
                    .parse::<usize>()
                    .unwrap();

                let inspection_count = 0;

                let monkey = Monkey {
                    items,
                    operation,
                    test_divisor,
                    true_monkey,
                    false_monkey,
                    inspection_count,
                };
                monkeys.push(monkey);
            }
        } else {
            // EOF
            break;
        }
    }
    monkeys
}

fn process_item(start_val: usize, operation: &Operation, div_factor: usize) -> usize {
    let mut end_val: usize = start_val;
    // Perform the operation on the item
    match operation {
        Operation::Add(num) => end_val += num,
        Operation::Multiply(num) => end_val *= num,
        Operation::Double() => end_val *= 2,
        Operation::Square() => end_val *= end_val,
    }

    // Take the floor of dividing by 3, which in Rust is just the result of
    // doing integer division on usizes
    // Uncomment this for Part 1
    // end_val = end_val / 3;

    // This keeps us from overflowing with multiplications; we only care about
    // divisibility, and div_factor is all the monkey's div factors multiplied
    // together
    end_val % div_factor
}

fn process_round(monkeys: &mut Vec<Monkey>, div_factor: usize) -> () {
    // Use a range here so we can do some tricks with borrowing. The issue here
    // is that nothing in our code guarentees that a monkey never throws to
    // itself, so naively pushing into the true/false monkey's item stack
    // *could* be a second mutable borrow of the same monkey.
    for monkey_num in 0..monkeys.len() {
        let mut true_items = VecDeque::<usize>::new();
        let mut false_items = VecDeque::<usize>::new();
        {
            let mut monkey = &mut monkeys[monkey_num];
            let items = &mut monkey.items;
            let operation = &monkey.operation;
            while items.len() != 0 {
                let item = items.pop_front().unwrap();
                let new_item = process_item(item, operation, div_factor);

                if new_item % monkey.test_divisor == 0 {
                    true_items.push_back(new_item);
                } else {
                    false_items.push_back(new_item);
                }

                monkey.inspection_count += 1;
            }
        }
        let true_monkey = monkeys[monkey_num].true_monkey;
        let false_monkey = monkeys[monkey_num].false_monkey;
        monkeys[true_monkey].items.append(&mut true_items);
        monkeys[false_monkey].items.append(&mut false_items);

        // Since we're not handling self-throws (we'd need to explicitly
        // re-process this monkey if it still held items), assert it
        assert!(monkeys[monkey_num].items.len() == 0);
    }
}

fn main() {
    let mut monkeys: Vec<Monkey> = get_monkeys_from_file("input.txt");

    let mut div_factor = 1usize;
    for monkey in &monkeys {
        div_factor *= monkey.test_divisor;
    }

    // Go 10K rounds. For Part 1, use 20 here instead.
    for _ in 0..10000 {
        process_round(&mut monkeys, div_factor);
    }

    // Multiply the two largest inspection counts
    let mut counts = BinaryHeap::<usize>::new();
    for monkey in monkeys {
        counts.push(monkey.inspection_count);
    }

    let best = counts.pop().unwrap();
    let second_best = counts.pop().unwrap();

    println!("{:?}", best * second_best);
}
