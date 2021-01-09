use anyhow::Result;
use itertools::Itertools;
use std::collections::VecDeque;
use std::fs::File;
use std::io::{self, prelude::*};
use std::path::Path;

#[derive(Debug)]
struct ProblemInvariantError;

impl std::error::Error for ProblemInvariantError {}

impl std::fmt::Display for ProblemInvariantError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "An invariant of the Advent of Code challenge seems to have been violated."
        )
    }
}

pub fn problem1() -> anyhow::Result<()> {
    let path = Path::new("input_day9.txt");
    let display = path.display();

    let file = match File::open(&path) {
        Err(why) => panic!("failed to open {}: {}", display, why),
        Ok(file) => file,
    };

    let numbers = io::BufReader::new(file)
        .lines()
        .collect::<Result<Vec<_>, _>>()?
        .iter()
        .map(|x| x.parse::<u64>())
        .collect::<Result<Vec<_>, _>>()?;

    let window_size = 25;
    let mut window = numbers.iter().take(window_size).collect::<VecDeque<_>>();
    let mut missing = None;
    for x in numbers.iter().skip(window_size) {
        let valid = window
            .iter()
            .tuple_combinations::<(_, _)>()
            .any(|(a, b)| *a + *b == *x);

        if valid {
            window.pop_front();
            window.push_back(x);
        } else {
            missing = Some(x);
            break;
        }
    }

    match missing {
        Some(x) => println!("{}", x),
        None => return Err(ProblemInvariantError.into()),
    }

    Ok(())
}

pub fn problem2() -> anyhow::Result<()> {
    let path = Path::new("input_day9.txt");
    let display = path.display();

    let file = match File::open(&path) {
        Err(why) => panic!("failed to open {}: {}", display, why),
        Ok(file) => file,
    };

    let numbers = io::BufReader::new(file)
        .lines()
        .collect::<Result<Vec<_>, _>>()?
        .iter()
        .map(|x| x.parse::<u64>())
        .collect::<Result<Vec<_>, _>>()?;

    let window_size = 25;
    let mut window = numbers.iter().take(window_size).collect::<VecDeque<_>>();
    let mut missing = None;
    for x in numbers.iter().skip(window_size) {
        let valid = window
            .iter()
            .tuple_combinations::<(_, _)>()
            .any(|(a, b)| *a + *b == *x);

        if valid {
            window.pop_front();
            window.push_back(x);
        } else {
            missing = Some(x);
            break;
        }
    }

    let missing = missing.ok_or(ProblemInvariantError)?;
    let mut bounds: Option<(u64, u64)> = None;
    'outer: for n in 2..numbers.len() {
        for window in numbers.windows(n) {
            if window.iter().sum::<u64>() == *missing {
                let min = window.iter().min().unwrap();
                let max = window.iter().max().unwrap();
                bounds = Some((*min, *max));
                break 'outer;
            }
        }
    }

    match bounds {
        Some((min, max)) => println!("{}", min + max),
        None => return Err(ProblemInvariantError.into()),
    }

    Ok(())
}
