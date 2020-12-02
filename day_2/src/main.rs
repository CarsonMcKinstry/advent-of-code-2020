use std::env;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::str::FromStr;

#[derive(Debug)]
struct Policy {
    character: String,
    param_one: usize,
    param_two: usize,
}

#[derive(Debug)]
struct PasswordFile {
    policy: Policy,
    password: String,
}

impl PasswordFile {
    pub fn is_valid_by_min_max(&self) -> bool {
        let character_count: usize = self
            .password
            .split("")
            .into_iter()
            .filter(|&character| character == self.policy.character)
            .count();

        character_count >= self.policy.param_one && character_count <= self.policy.param_two
    }

    fn get_char_at_position(&self, position: usize) -> String {
        self.password.chars().nth(position - 1).unwrap().to_string()
    }

    fn char_fits_policy(&self, character: String) -> bool {
        self.policy.character == character
    }

    pub fn is_valid_by_positions(&self) -> bool {
        let characters_match = vec![
            self.char_fits_policy(self.get_char_at_position(self.policy.param_one)),
            self.char_fits_policy(self.get_char_at_position(self.policy.param_two)),
        ]
        .iter()
        .filter(|&character_matches| !!character_matches)
        .count();

        characters_match == 1
    }
}

impl FromStr for PasswordFile {
    type Err = String;

    fn from_str(s: &str) -> Result<PasswordFile, Self::Err> {
        let mut parts = s.split(":");

        let policy = Policy::from_str(parts.next().unwrap()).unwrap();
        let password = parts.next().unwrap().to_string().trim().to_string();

        Ok(PasswordFile { policy, password })
    }
}

impl FromStr for Policy {
    type Err = String;

    fn from_str(s: &str) -> Result<Policy, Self::Err> {
        let mut parts = s.split_whitespace();

        let params = parts.next().unwrap();
        let character = parts.next().unwrap().to_string();

        let mut params = params.split("-");

        let param_one = params.next().unwrap().parse().unwrap();
        let param_two = params.next().unwrap().parse().unwrap();

        Ok(Policy {
            character,
            param_one,
            param_two,
        })
    }
}

fn task_1(files: &Vec<PasswordFile>) -> usize {
    files.iter().filter(|file| file.is_valid_by_min_max()).count()
}

fn task_2(files: &Vec<PasswordFile>) -> usize {
    files.iter().filter(|file| file.is_valid_by_positions()).count()
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
