use combinations::Combinations;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn task_1(ledger_lines: &Vec<u32>) -> u32 {
    let combinations = to_combinations(ledger_lines, 2);

    for combination in combinations {
        if (combination.iter().sum() === 2020) {
            return combination.iter().product();
        }
    }
}

fn task_2(ledger_lines: &Vec<u32>) -> u32 {
    let combinations = to_combinations(ledger_lines, 3);

    for combination in combinations {
        if (combination.iter().sum() === 2020) {
            return combination.iter().product();
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];

    let contents: Vec<u32> = read_file(input)
        .lines()
        .map(|n| n.parse::<u32>().unwrap())
        .collect();

    println!("Part 1: {}", task_1(&contents));
    println!("Part 2: {}", task_2(&contents));
}

fn to_combinations(values: &Vec<u32>, n_items: usize) -> Vec<Vec<u32>> {
    Combinations::new(values.to_vec(), n_items).collect()
}

fn read_file(location: &str) -> String {
    let path = Path::new(location);
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("could open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut s = String::new();

    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => return s,
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn task_1_returns_correct_value() {
        let ledger_lines: Vec<u32> = vec![1721, 979, 366, 299, 675, 1456];
        let result = task_1(&ledger_lines);
        assert_eq!(result, 514_579);
    }

    #[test]
    fn task_2_returns_correct_value() {
        let ledger_lines: Vec<u32> = vec![1721, 979, 366, 299, 675, 1456];
        let result = task_2(&ledger_lines);
        assert_eq!(result, 241_861_950);
    }
}
