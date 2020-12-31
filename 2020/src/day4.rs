use anyhow::Result;
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, prelude::*};
use std::path::Path;

#[allow(dead_code)]
struct Passport {
    byr: String,
    iyr: String,
    eyr: String,
    hgt: String,
    hcl: String,
    ecl: String,
    pid: String,
    cid: Option<String>,
}

pub fn problem1() -> anyhow::Result<()> {
    let path = Path::new("input_day4.txt");
    let display = path.display();

    let file = match File::open(&path) {
        Err(why) => panic!("failed to open {}: {}", display, why),
        Ok(file) => file,
    };

    let passports = parse_passports(&file)?;
    println!("{}", passports.len());
    Ok(())
}

pub fn problem2() -> anyhow::Result<()> {
    let path = Path::new("input_day4.txt");
    let display = path.display();

    let file = match File::open(&path) {
        Err(why) => panic!("failed to open {}: {}", display, why),
        Ok(file) => file,
    };

    let num_passports = parse_passports(&file)?
        .into_iter()
        .filter(validate_passport)
        .count();
    println!("{}", num_passports);
    Ok(())
}

fn parse_passports(file: &File) -> anyhow::Result<Vec<Passport>> {
    let lines = io::BufReader::new(file)
        .lines()
        .collect::<Result<Vec<_>, _>>()?;

    let groups = lines.split(|line| line.is_empty());
    let passports = groups
        .map(|group| parse_group(&group.to_vec()))
        .filter_map(|passport| passport)
        .collect::<Vec<Passport>>();
    Ok(passports)
}

fn parse_group(group: &[String]) -> Option<Passport> {
    let kvps = group
        .iter()
        .map(|line| line.split(' ').collect::<Vec<_>>())
        .collect::<Vec<_>>()
        .concat();

    let fields = kvps
        .iter()
        .map(|kvp| kvp.split(':'))
        .map(|mut arr| (arr.next().zip(arr.next())))
        .collect::<Option<Vec<_>>>()?
        .into_iter()
        .collect::<HashMap<_, _>>();

    let passport = Passport {
        byr: (*fields.get("byr")?).to_string(),
        iyr: (*fields.get("iyr")?).to_string(),
        eyr: (*fields.get("eyr")?).to_string(),
        hgt: (*fields.get("hgt")?).to_string(),
        hcl: (*fields.get("hcl")?).to_string(),
        ecl: (*fields.get("ecl")?).to_string(),
        pid: (*fields.get("pid")?).to_string(),
        cid: fields.get("cid").map(|x| x.to_string()),
    };
    Some(passport)
}

fn validate_passport(passport: &Passport) -> bool {
    validate_byr(passport)
        && validate_iyr(passport)
        && validate_eyr(passport)
        && validate_hgt(passport)
        && validate_hcl(passport)
        && validate_ecl(passport)
        && validate_pid(passport)
}

fn validate_byr(passport: &Passport) -> bool {
    match passport.byr.parse::<u32>() {
        Err(_) => false,
        Ok(x) => 1920 <= x && x <= 2002,
    }
}

fn validate_iyr(passport: &Passport) -> bool {
    match passport.iyr.parse::<u32>() {
        Err(_) => false,
        Ok(x) => 2010 <= x && x <= 2020,
    }
}

fn validate_eyr(passport: &Passport) -> bool {
    match passport.eyr.parse::<u32>() {
        Err(_) => false,
        Ok(x) => 2020 <= x && x <= 2030,
    }
}

fn validate_hgt(passport: &Passport) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^(\d+)(cm|in)$").unwrap();
    }
    let _groups = RE.captures(&passport.hgt);
    match _groups {
        None => false,
        Some(groups) => {
            let value = match groups.get(1).unwrap().as_str().parse::<u32>() {
                Ok(x) => x,
                Err(_) => return false,
            };
            let unit = groups.get(2).unwrap().as_str();
            match unit {
                "cm" => 150 <= value && value <= 193,
                "in" => 59 <= value && value <= 76,
                _ => false,
            }
        }
    }
}

fn validate_hcl(passport: &Passport) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    }
    RE.is_match(&passport.hcl)
}

fn validate_ecl(passport: &Passport) -> bool {
    match passport.ecl.as_str() {
        "amb" => true,
        "blu" => true,
        "brn" => true,
        "gry" => true,
        "grn" => true,
        "hzl" => true,
        "oth" => true,
        _ => false,
    }
}

fn validate_pid(passport: &Passport) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^[0-9]{9}$").unwrap();
    }
    RE.is_match(&passport.pid)
}
