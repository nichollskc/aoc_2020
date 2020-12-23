use regex::Regex;
use std::fs;

#[derive(Debug)]
struct Password {
    password: String,
    min_occ: usize,
    max_occ: usize,
    required_char: char,
}

impl Password {
    fn new(database_entry: &str) -> Self {
        let re = Regex::new(r"(\d+)-(\d+) ([a-z]): ([a-z]*)").unwrap();   
        let caps = re.captures(database_entry).unwrap();
        Password {
            password: caps[4].to_owned(),
            min_occ: caps[1].to_owned().parse().expect("Issue reading"),
            max_occ: caps[2].to_owned().parse().expect("Issue reading"),
            required_char: caps[3].to_owned().parse().expect("Issue reading"),
        }
    }

    fn is_valid(&self) -> bool {
        let num_occ = self.password.matches(self.required_char).count();
        return (num_occ >= self.min_occ) & (num_occ <= self.max_occ)
    }

    fn is_valid_new_rules(&self) -> bool {
        let first_char_matches: u8 = self.nth_character_matches_req(self.min_occ).into();
        let last_char_matches: u8 = self.nth_character_matches_req(self.max_occ).into();
        let num_matches: u8 = first_char_matches + last_char_matches;
        num_matches == 1
    }

    fn nth_character_matches_req(&self, index: usize) -> bool {
        let nth = self.password.chars().nth(index - 1);
        match nth {
            Some(c) => c == self.required_char,
            None => false,
        }
    }
}

fn main() {
    let passwords = read_passwords("input/day2.txt");
    let mut count_valid = 0;
    let mut count_valid_new_rules = 0;
    for password in passwords {
        println!("Password: {:?}", password);
        if password.is_valid() {
            count_valid += 1;
            println!("Valid");
        }
        if password.is_valid_new_rules() {
            count_valid_new_rules += 1;
            println!("Valid by new rules");
        }
    }
    println!("Number of valid passwords is: {}", count_valid);
    println!("Number of valid passwords by new rules is: {}", count_valid_new_rules);
}

fn read_passwords(filename: &str) -> Vec<Password> {
    let contents = fs::read_to_string(filename).expect("Error reading file");
    let mut passwords: Vec<Password> = Vec::new();
    for line in contents.lines() {
        let password = Password::new(line);
        passwords.push(password);
    }
    passwords
}
