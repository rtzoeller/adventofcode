use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, self};
use std::path::Path;
use anyhow::Result;

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