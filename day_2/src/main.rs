use std::env;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::str::FromStr;

#[derive(Debug)]
struct Policy {
    character: String,
    min: u32,
    max: u32,
}

#[derive(Debug)]
struct PasswordFile {
    policy: Policy,
    password: String,
}

impl PasswordFile {
    pub fn is_valid_1(&self) -> bool {
        let char_count: u32 = self
            .password
            .split("")
            .into_iter()
            .filter(|&c| c == self.policy.character)
            .count() as u32;

        char_count >= self.policy.min && char_count <= self.policy.max
    }

    fn get_char_at_position(&self, position: usize) -> String {
        self.password.chars().nth(position - 1).unwrap().to_string()
    }

    pub fn is_valid_2(&self) -> bool {
        let pos_one = self.get_char_at_position(self.policy.min as usize);
        let pos_two = self.get_char_at_position(self.policy.max as usize);

        let pos_one_matches = pos_one == self.policy.character;
        let pos_two_matches = pos_two == self.policy.character;

        (pos_one_matches && !pos_two_matches) || (!pos_one_matches && pos_two_matches)
    }
}

impl FromStr for PasswordFile {
    type Err = String;

    fn from_str(s: &str) -> Result<PasswordFile, Self::Err> {
        let mut parts = s.split(":");

        let policy = parts.next().unwrap();
        let password = parts.next().unwrap().to_string().trim().to_string();

        let mut policy_parts = policy.split_whitespace();

        let num = policy_parts.next().unwrap();
        let c = policy_parts.next().unwrap();

        let mut min_max = num.split("-");

        let min = min_max.next().unwrap().parse::<u32>().unwrap();
        let max = min_max.next().unwrap().parse::<u32>().unwrap();

        let policy = Policy {
            min,
            max,
            character: c.to_string(),
        };

        Ok(PasswordFile { policy, password })
    }
}

fn task_1(files: &Vec<PasswordFile>) -> usize {
    files.iter().filter(|file| file.is_valid_1()).count()
}

fn task_2(files: &Vec<PasswordFile>) -> usize {
    files.iter().filter(|file| file.is_valid_2()).count()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];

    let contents: Vec<PasswordFile> = read_file(input)
        .lines()
        .map(|n| PasswordFile::from_str(n))
        .map(|n| n.unwrap())
        .collect();

    println!("Task 1: {}", task_1(&contents));
    println!("Task 2: {}", task_2(&contents));
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

    fn test_passwords() -> Vec<PasswordFile> {
        vec![
            PasswordFile::from_str("1-3 a: abcde").unwrap(),
            PasswordFile::from_str("1-3 b: cdefg").unwrap(),
            PasswordFile::from_str("2-9 c: ccccccccc").unwrap(),
        ]
    }

    #[test]
    fn test_task_1() {
        let test_passwords = test_passwords();

        assert_eq!(task_1(&test_passwords), 2);
    }

    #[test]
    fn test_task_2() {
        let test_passwords = test_passwords();

        assert_eq!(task_2(&test_passwords), 1);
    }
}
