use std::convert::TryFrom;
use std::fs::File;
use std::io::{prelude::*, self};
use std::num::ParseIntError;
use std::path::Path;
use regex::Regex;

pub fn problem1() -> Result<(), ParseIntError> {
    let path = Path::new("input_day2.txt");
    let display = path.display();

    let file = match File::open(&path) {
        Err(why) => panic!("failed to open {}: {}", display, why),
        Ok(file) => file
    };

    let lines = io::BufReader::new(file).lines();
    let count = lines.map(|line| line.unwrap())
                     .filter(|line| problem1_validate_line(&line))
                     .count();

    println!("{}", count);
    Ok(())
}

fn problem1_validate_line(s: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^(\d+)-(\d+) ([a-z]): ([a-z]+)$").unwrap();
    }
    let _groups = RE.captures(s);

    match _groups {
        None => panic!("Regex didn't match."),
        Some(groups) => {
            let lower = groups.get(1).unwrap().as_str().parse::<u32>().unwrap();
            let upper = groups.get(2).unwrap().as_str().parse::<u32>().unwrap();
            let character = groups.get(3).unwrap().as_str().chars().next().unwrap();
            let password = groups.get(4).unwrap().as_str();
            let count = u32::try_from(password.matches(character).count()).unwrap();

            lower <= count && count <= upper
        },
    }
}

pub fn problem2() -> Result<(), ParseIntError> {
    let path = Path::new("input_day2.txt");
    let display = path.display();

    let file = match File::open(&path) {
        Err(why) => panic!("failed to open {}: {}", display, why),
        Ok(file) => file
    };

    let lines = io::BufReader::new(file).lines();
    let count = lines.map(|line| line.unwrap())
                     .filter(|line| problem2_validate_line(&line))
                     .count();

    println!("{}", count);
    Ok(())
}

fn problem2_validate_line(s: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^(\d+)-(\d+) ([a-z]): ([a-z]+)$").unwrap();
    }
    let _groups = RE.captures(s);

    match _groups {
        None => panic!("Regex didn't match."),
        Some(groups) => {
            let first = groups.get(1).unwrap().as_str().parse::<u32>().unwrap();
            let second = groups.get(2).unwrap().as_str().parse::<u32>().unwrap();
            let character = groups.get(3).unwrap().as_str().chars().next().unwrap();
            let password = groups.get(4).unwrap().as_str();

            let first_match = password.chars().nth(usize::try_from(first - 1).unwrap()).unwrap() == character;
            let second_match = password.chars().nth(usize::try_from(second - 1).unwrap()).unwrap() == character;

            first_match ^ second_match
        },
    }
}
