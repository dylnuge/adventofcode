// Advent of Code 2022 Day 8 Part 2

use std::fs;

fn main() {
    let data: String = fs::read_to_string("input.txt").unwrap();
    let rows: usize = data.lines().count();
    let cols: usize = data.lines().next().unwrap().len();

    // Build data array. vals are all single digits, so won't exceed a u8
    let mut values = vec![vec![0u8; cols]; rows];

    // Parse the data into a 2D array of ints.
    for (row, line) in data.lines().enumerate() {
        for (col, value) in line.chars().enumerate() {
            values[row][col] = value.to_digit(10).unwrap() as u8;
        }
    }

    // The naïve solution here is cubic on edge length (each cell must check
    // its row and column). It's likely possible to improve this, but I didn't
    // quickly think of a way.
    let mut highest_scenic: usize = 0;
    for row in 0..rows {
        for col in 0..cols {
            let tree = values[row][col];
            // Check up
            let mut vis_up = 0;
            for inner_row in (0..row).rev() {
                vis_up += 1;
                if values[inner_row][col] >= tree {
                    break;
                }
            }
            // Check down
            let mut vis_down = 0;
            for inner_row in row + 1..rows {
                vis_down += 1;
                if values[inner_row][col] >= tree {
                    break;
                }
            }
            // Check left
            let mut vis_left = 0;
            for inner_col in (0..col).rev() {
                vis_left += 1;
                if values[row][inner_col] >= tree {
                    break;
                }
            }
            // Check right
            let mut vis_right = 0;
            for inner_col in col + 1..cols {
                vis_right += 1;
                if values[row][inner_col] >= tree {
                    break;
                }
            }

            let scenic = vis_left * vis_right * vis_up * vis_down;
            if scenic > highest_scenic {
                highest_scenic = scenic;
            }
        }
    }

    println!("Highest scenic value: {}", highest_scenic);
}
