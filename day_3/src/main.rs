use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();

    let input = get_contents_from_input(&args[1]);

    println!("Task 1: {}", task_1(&input));
    println!("Task 2: {}", task_2(&input));
}

fn task_1(input: &Vec<Vec<String>>) -> u32 {
    hits_on_slope(input, 1, 3) as u32
}

fn task_2(input: &Vec<Vec<String>>) -> u64 {
    (1..=7)
        .step_by(2)
        .fold(1, |acc, run| acc * hits_on_slope(input, 1, run))
        * hits_on_slope(input, 2, 1)
}

fn hits_on_slope(lines: &Vec<Vec<String>>, rise: u32, run: u32) -> u64 {
    let mut hit_count: u64 = 0;
    let mut x_pos: u32 = 0;

    for line in lines.iter().skip(1).step_by(rise as usize) {
        x_pos += run;
        x_pos = x_pos % line.len() as u32;
        let maybe_tree = &line[x_pos as usize];
        if maybe_tree == "#" {
            hit_count += 1;
        }
    }

    hit_count
}

fn get_contents_from_input(input: &str) -> Vec<Vec<String>> {
    let contents = read_file(input);

    let mut lines: Vec<Vec<String>> = vec![];

    for line in contents.lines() {
        let line = line
            .trim()
            .split("")
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .collect();

        lines.push(line);
    }

    lines
}

fn read_file(location: &str) -> String {
    let path = Path::new(location);
    let display = path.display();

    match fs::read_to_string(location) {
        Err(why) => panic!("Couldn't read {}: {}", display, why),
        Ok(s) => s,
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn task_1_should_return_7_for_test_input() {
        let contents = get_contents_from_input("testInput.txt");

        assert_eq!(task_1(&contents), 7);
    }
    #[test]
    fn task_2_should_return_336_for_test_input() {
        let contents = get_contents_from_input("testInput.txt");

        assert_eq!(task_2(&contents), 336);
    }
}
