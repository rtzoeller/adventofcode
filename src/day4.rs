use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, self};
use std::path::Path;
use anyhow::Result;
use regex::Regex;

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
        Ok(file) => file
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
        Ok(file) => file
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
    let passports = groups.map(|group| parse_group(&group.to_vec()))
                          .filter_map(|passport| passport)
                          .collect::<Vec<Passport>>();
    Ok(passports)
}

fn parse_group(group: &Vec<String>) -> Option<Passport> {
    let kvps = group.into_iter()
                        .map(|line| line.split(" ").collect::<Vec<_>>())
                        .collect::<Vec<_>>()
                        .concat();

    let fields = kvps.into_iter()
                        .map(|kvp| kvp.split(":"))
                        .map(|mut arr| (arr.next().zip(arr.next())))
                        .collect::<Option<Vec<_>>>()?
                        .into_iter()
                        .collect::<HashMap<_, _>>();

    let passport = Passport {
        byr: fields.get("byr")?.clone().to_string(),
        iyr: fields.get("iyr")?.clone().to_string(),
        eyr: fields.get("eyr")?.clone().to_string(),
        hgt: fields.get("hgt")?.clone().to_string(),
        hcl: fields.get("hcl")?.clone().to_string(),
        ecl: fields.get("ecl")?.clone().to_string(),
        pid: fields.get("pid")?.clone().to_string(),
        cid: fields.get("cid").clone().and_then(|x| Some(x.to_string())),
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
        },
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
