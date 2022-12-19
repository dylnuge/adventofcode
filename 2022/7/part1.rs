// Advent of Code 2022 Day 7 Part 1

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

mod fs_util;

enum Command {
    Unknown,
    // Our simplified vocab only supports ls on the current directory
    Ls,
    Cd(String),
}

fn parse_command(prompt: &str) -> Command {
    let components: Vec<&str> = prompt.split(' ').collect();
    assert!(components.len() > 1);
    assert!(components[0] == "$");

    match components[1] {
        "ls" => Command::Ls,
        "cd" => {
            if components.len() >= 3 {
                Command::Cd(components[2].to_owned())
            } else {
                // We weren't given a definition for bare cd, though cd / could
                // be another interpretation
                Command::Unknown
            }
        }
        _ => Command::Unknown,
    }
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut fs = fs_util::make_fs();

    let mut is_ls_output: bool = false;
    // We know the input opens with a "cd /", but we'll "start" in the root
    // directory anyways for good measure
    let mut current_path: String = "/".to_owned();

    for line_result in reader.lines() {
        let line = line_result.unwrap();

        // If the first character is '$' we are reading a prompt line
        let is_command: bool = line.as_bytes()[0] == '$' as u8;

        if is_command {
            // prompt lines end command output
            is_ls_output = false;
            let command = parse_command(&line);

            match command {
                Command::Ls => {
                    // expect ls output until next command
                    is_ls_output = true;
                }
                Command::Cd(path) => {
                    current_path = fs_util::get_path_from_relative(&current_path, &path);
                }
                Command::Unknown => {
                    panic!("Bad command or filename");
                }
            }
        } else if is_ls_output {
            let components: Vec<&str> = line.split(' ').collect();
            assert!(components.len() == 2);
            match (components[0], components[1]) {
                ("dir", dir_name) => {
                    let dir_path = fs_util::get_path_from_relative(&current_path, &dir_name);
                    fs.mkdir(&dir_path);
                }
                (size, file_name) => {
                    let size_val = size.parse::<usize>().unwrap();
                    fs.mkfile(&current_path, &file_name, size_val);
                }
            }
        } else {
            // We should either have a command or be reading the output from one
            panic!("Unexpected input: {}", line)
        }
    }

    let mut size: usize = 0;
    for node in &fs.nodes {
        if let (_, fs_util::FsEntry::Directory(dir_entry)) = node {
            let dir_size = fs.get_dir_size(&dir_entry);
            if dir_size <= 100000 {
                size += dir_size;
            }
        }
    }

    println!("Size: {}", size);
}
