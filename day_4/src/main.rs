use itertools::Itertools;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::Path;
use std::str;

const REQUIRED_PASSPORT_FIELDS: [&str; 7] = [
    "byr", // Birth Year
    "iyr", // Issue Year
    "eyr", // Expiration Year
    "hgt", // Height
    "hcl", // Hair Color
    "ecl", // Eye Color
    "pid", // Passport ID
           // "cid", // Country ID
];

const ACCEPTABLE_EYE_COLORS: [&str; 7] = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

// byr (Birth Year) - four digits; at least 1920 and at most 2002.
// iyr (Issue Year) - four digits; at least 2010 and at most 2020.
// eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
// hgt (Height) - a number followed by either cm or in:
// If cm, the number must be at least 150 and at most 193.
// If in, the number must be at least 59 and at most 76.
// hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
// ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
// pid (Passport ID) - a nine-digit number, including leading zeroes.
// cid (Country ID) - ignored, missing or not.

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];

    let passports = file_to_passport_strings(&read_file(&input));

    let parsed_passports = task_1(&passports);
    println!("Task 1: {}", parsed_passports.len());
    println!("Task 2: {}", task_2(&parsed_passports));
}

fn task_1(passports: &Vec<String>) -> Vec<HashMap<&str, &str>> {
    passports
        .iter()
        .filter_map(|p| parse_passport(&p))
        .collect()
}

fn task_2(passports: &Vec<HashMap<&str, &str>>) -> usize {
    passports
        .iter()
        .filter(|passport| is_valid_passport(&passport))
        .count()
}

fn read_file(location: &str) -> String {
    let path = Path::new(location);
    let display = path.display();

    match fs::read_to_string(location) {
        Err(why) => panic!("Couldn't read {}: {}", display, why),
        Ok(s) => s,
    }
}

fn parse_passport(s: &str) -> Option<HashMap<&str, &str>> {
    let passport = s
        .split_whitespace()
        .flat_map(|p| p.split(':'))
        .tuples()
        .collect::<HashMap<_, _>>();

    let invalid = REQUIRED_PASSPORT_FIELDS
        .iter()
        .any(|key| !passport.contains_key(key));

    if invalid {
        None
    } else {
        Some(passport)
    }
}

// byr (Birth Year) - four digits; at least 1920 and at most 2002.
// iyr (Issue Year) - four digits; at least 2010 and at most 2020.
// eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
// hgt (Height) - a number followed by either cm or in:
// If cm, the number must be at least 150 and at most 193.
// If in, the number must be at least 59 and at most 76.
// hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
// ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
// pid (Passport ID) - a nine-digit number, including leading zeroes.
// cid (Country ID) - ignored, missing or not.

fn is_valid_passport(passport: &HashMap<&str, &str>) -> bool {
    passport.iter().all(|(&key, value)| match key {
        "byr" => (1920..=2002).contains(&value.parse::<u32>().unwrap()),
        "iyr" => (2010..=2020).contains(&value.parse::<u32>().unwrap()),
        "eyr" => (2020..=2030).contains(&value.parse::<u32>().unwrap()),
        "hcl" => {
            value.starts_with("#")
                && value.len() == 7
                && value.chars().skip(1).all(|c| c.is_ascii_hexdigit())
        }
        "ecl" => ACCEPTABLE_EYE_COLORS.contains(value),
        "pid" => value.len() == 9 && value.chars().all(|c| c.is_ascii_digit()),
        "hgt" => {
            let height: u32 = value[0..(value.len()-2)].parse().unwrap_or(0);
            let unit: &str = &value[(value.len()-2)..value.len()];

            match unit {
                "cm" => (150..=193).contains(&height),
                "in" => (59..=76).contains(&height),
                _ => false
            }
        },
        "cid" => true,
        _ => unreachable!(),
    })
}

fn file_to_passport_strings(file: &str) -> Vec<String> {
    let mut passports: Vec<String> = vec![];

    let mut current_passport = String::default();

    for line in file.lines() {
        if line.len() > 0 {
            current_passport += line;
            current_passport += " ";
        } else {
            passports.push(current_passport);
            current_passport = String::default();
        }
    }

    passports.push(current_passport);

    passports
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn task_1_returns_2_for_test_input() {
        let passports = read_file("testInput.txt");

        assert_eq!(task_1(&file_to_passport_strings(&passports)).len(), 2);
    }

    #[test]
    fn task_2_returns_4_for_test_input() {
        let passports = file_to_passport_strings(&read_file("testInput2.txt"));
        let parsed_passports = task_1(&passports);

        assert_eq!(task_2(&parsed_passports), 4);
        
    }
}
