use std::env;
use std::fs;

fn task_1(ledger_lines: Vec<u32>) -> u32 {

    let mut total: u32 = 0;

    for (i, line) in ledger_lines.iter().enumerate() {
        let lines_to_check = &ledger_lines[i + 1..];

        if lines_to_check.len() > 0 {
            for line_to_check in lines_to_check {
                if line + line_to_check == 2020 {
                    total = line * line_to_check;
                }
            }
        }
    }

    total
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];

    let contents = fs::read_to_string(input).expect("Something went wrong reading the file");

    let ledger_lines = contents
        .split("\n")
        .collect::<Vec<&str>>()
        .iter()
        .map(|n| n.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    println!("Task 1: {}", task_1(ledger_lines));
}
