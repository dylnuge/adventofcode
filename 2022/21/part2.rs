// Advent of Code 2022 Day 21 Part 2

use std::collections::HashMap;
use std::fs;

// Given an equation of the form "monkey = X",
fn process_variable(
    monkeys: &HashMap<String, String>,
    monkey_data: &HashMap<String, (String, String, String)>,
    var_side_name: &str,
    mut current_val: isize,
) -> isize {
    // This is the base case: humn = current_val
    if var_side_name == "humn" {
        return current_val;
    }

    // Get the results from when the monkey was processed and determine which
    // side has the variable and what monkey produces it
    let instruction = monkeys.get(var_side_name).unwrap();
    let elems: Vec<&str> = instruction.split_whitespace().collect();
    let (left_str, op, right_str) = monkey_data.get(var_side_name).unwrap();

    let left_val = left_str.parse::<isize>();
    let right_val = right_str.parse::<isize>();

    if let Err(_) = left_val {
        // X (op) val
        let mut val = right_val.unwrap();
        match op.as_str() {
            "+" => {
                current_val -= val;
            }
            "-" => {
                current_val += val;
            }
            "*" => {
                current_val = current_val / val;
            }
            "/" => {
                current_val *= val;
            }
            _ => panic!("bad operation {}", elems[1]),
        }
        return process_variable(monkeys, monkey_data, elems[0], current_val);
    } else if let Err(_) = right_val {
        // val (op) X; makes a difference for subtraction and division!
        let mut val = left_val.unwrap();
        match op.as_str() {
            "+" => {
                current_val -= val;
            }
            "-" => {
                current_val = val - current_val;
            }
            "*" => {
                current_val = current_val / val;
            }
            "/" => {
                current_val *= (1 / val);
            }
            _ => panic!("bad operation {}", elems[1]),
        }
        return process_variable(monkeys, monkey_data, elems[2], current_val);
    } else {
        // val (op) val; we shouldn't actually be here so panic
        panic!("no variable to process on monkey: {}", var_side_name);
    }
}

fn process_monkey(
    name: &str,
    monkeys: &HashMap<String, String>,
    monkey_data: &mut HashMap<String, (String, String, String)>,
) -> String {
    let instruction = monkeys.get(name).unwrap();

    // Force "humn" to be a variable
    if name == "humn" {
        return name.to_string();
    }

    if let Ok(_) = instruction.parse::<isize>() {
        return instruction.to_string();
    }

    let elems: Vec<&str> = instruction.split_whitespace().collect();
    assert!(elems.len() == 3);
    let left_str = process_monkey(elems[0], monkeys, monkey_data);
    let right_str = process_monkey(elems[2], monkeys, monkey_data);
    monkey_data.insert(
        name.to_string(),
        (
            left_str.to_string(),
            elems[1].to_string(),
            right_str.to_string(),
        ),
    );

    // If both sides are numbers, not variables, calculate their result
    if let Ok(left) = left_str.parse::<isize>() {
        if let Ok(right) = right_str.parse::<isize>() {
            return match elems[1] {
                "+" => format!("{}", left + right),
                "-" => format!("{}", left - right),
                "*" => format!("{}", left * right),
                "/" => format!("{}", left / right),
                _ => panic!("bad operation {}", elems[1]),
            };
        }
    }

    // If we hit the root, it's time to dive back down
    if name == "root" {
        let mut result: isize = 0;
        let mut variable_monkey: &str;

        // Find which side of the "equation" has a variable answer still
        if let Ok(left_val) = left_str.parse::<isize>() {
            result = left_val;
            variable_monkey = elems[2];
        } else {
            let right_val = right_str.parse::<isize>().unwrap();
            result = right_val;
            variable_monkey = elems[0];
        }

        result = process_variable(monkeys, &monkey_data, variable_monkey, result);
        return format!("human should be: {}", result);
    }

    // This is mostly for debugging to "build" the equation. Practically,
    // we don't care.
    return match elems[1] {
        "+" => format!("({} + {})", left_str, right_str),
        "-" => format!("({} - {})", left_str, right_str),
        "*" => format!("({} * {})", left_str, right_str),
        "/" => format!("({} / {})", left_str, right_str),
        _ => panic!("bad operation {}", elems[1]),
    };
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

    // For memoization; stores results from the "first" pass to look up when
    // doing the equation settling
    let mut monkey_data: HashMap<String, (String, String, String)> = HashMap::new();
    println!("{:?}", process_monkey("root", &monkeys, &mut monkey_data));
}
