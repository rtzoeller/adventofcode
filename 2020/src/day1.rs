use std::fs::File;
use std::io::{self, prelude::*};
use std::num::ParseIntError;
use std::path::Path;

pub fn problem1() -> Result<(), ParseIntError> {
    let path = Path::new("input_day1.txt");
    let display = path.display();

    let file = match File::open(&path) {
        Err(why) => panic!("failed to open {}: {}", display, why),
        Ok(file) => file,
    };

    let lines = io::BufReader::new(file).lines();
    let mut numbers = Vec::new();
    for _line in lines {
        if let Ok(line) = _line {
            let number = line.parse::<u64>()?;
            numbers.push(number);
        }
    }

    for x in &numbers {
        for y in &numbers {
            if x + y == 2020 {
                println!("{}", x * y);
                return Ok(());
            }
        }
    }

    Ok(())
}

pub fn problem2() -> Result<(), ParseIntError> {
    let path = Path::new("input_day1.txt");
    let display = path.display();

    let file = match File::open(&path) {
        Err(why) => panic!("failed to open {}: {}", display, why),
        Ok(file) => file,
    };

    let lines = io::BufReader::new(file).lines();
    let mut numbers = Vec::new();
    for _line in lines {
        if let Ok(line) = _line {
            let number = line.parse::<u64>()?;
            numbers.push(number);
        }
    }

    for x in &numbers {
        for y in &numbers {
            for z in &numbers {
                if x + y + z == 2020 {
                    println!("{}", x * y * z);
                    return Ok(());
                }
            }
        }
    }

    Ok(())
}
