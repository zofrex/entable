extern crate regex;

use regex::Regex;
use std::cmp;
use std::io;
use std::io::Read;
use std::iter;

fn main() {
    let divider_re = Regex::new(r"^\+[\-\+]*\+$").unwrap();

    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Something went wrong reading from stdin, somehow");

    let lines: Vec<&str> = input.split_terminator("\n").collect();

    let mut counts: Vec<usize> = vec![];

    // First pass: determine column widths sufficient to contain all data
    for line in lines.clone() {
        let split_char = if divider_re.is_match(line) { "+" } else { "|" };

        let this_line_counts = line.split(split_char).map(|s| s.len());
        counts = this_line_counts
            .zip(counts.into_iter().chain(iter::repeat(0)))
            .map(|(previous_max, current_size)| cmp::max(previous_max, current_size))
            .collect();
    }

    // Second pass: print out tabulated columns
    for line in lines {
        if divider_re.is_match(line) {
            for c in counts.clone() {
                if c == 0 {
                    continue;
                }
                print!("+");
                for _ in 0..c {
                    print!("-");
                }
            }
            println!("+");
        } else {
            let data = line.split("|");
            for (i, d) in data.enumerate() {
                if d.len() == 0 {
                    continue;
                }
                print!("| {:1$}", d.trim(), counts[i] - 1);
            }
            println!("|");
        }
    }
}
