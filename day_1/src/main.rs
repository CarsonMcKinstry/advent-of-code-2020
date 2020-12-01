use std::env;
use std::fs;

fn task_1(ledger_lines: &Vec<u32>) -> u32 {
    let mut answer: u32 = 0;

    for (i, line) in ledger_lines.iter().enumerate() {
        let lines_to_check = &ledger_lines[i + 1..];

        if lines_to_check.len() > 0 {
            for line_to_check in lines_to_check {
                if line + line_to_check == 2020 {
                    answer = line * line_to_check;
                }
            }
        }
    }

    answer
}

fn task_2(ledger_lines: &Vec<u32>) -> u32 {
    let mut answer: u32 = 0;

    for (i, line) in ledger_lines.iter().enumerate() {
        let lines_to_check = &ledger_lines[i + 1..];

        if lines_to_check.len() > 0 {
            for (j, line_to_check) in lines_to_check.iter().enumerate() {
                let lines_to_check_2 = &lines_to_check[j + 1..];

                for line_to_check_2 in lines_to_check_2 {
                    if line + line_to_check + line_to_check_2 == 2020 {
                        answer = line * line_to_check * line_to_check_2;
                    }
                }
            }
        }
    }

    answer
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

    println!("Task 1: {}", task_1(&ledger_lines));
    println!("Task 2: {}", task_2(&ledger_lines));
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn task_1_returns_correct_value() {
        let ledger_lines: Vec<u32> = vec![1721, 979, 366, 299, 675, 1456];
        let result = task_1(&ledger_lines);
        assert_eq!(result, 514579);
    }

    #[test]
    fn task_2_returns_correct_value() {
        let ledger_lines: Vec<u32> = vec![1721, 979, 366, 299, 675, 1456];
        let result = task_2(&ledger_lines);
        assert_eq!(result, 241861950);
    }
}
