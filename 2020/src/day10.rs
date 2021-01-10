use anyhow::Result;
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
    let path = Path::new("input_day10.txt");
    let display = path.display();

    let file = match File::open(&path) {
        Err(why) => panic!("failed to open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut numbers = io::BufReader::new(file)
        .lines()
        .collect::<Result<Vec<_>, _>>()?
        .iter()
        .map(|x| x.parse::<u64>())
        .collect::<Result<Vec<_>, _>>()?;
    numbers.push(0);
    numbers.sort_unstable();
    numbers.push(numbers.last().unwrap() + 3);

    let differences = numbers
        .windows(2)
        .into_iter()
        .map(|xs| xs[1] - xs[0])
        .collect::<Vec<_>>();

    let ones = differences.iter().filter(|&&x| x == 1).count();
    let threes = differences.iter().filter(|&&x| x == 3).count();
    println!("{}", ones * threes);

    Ok(())
}

pub fn problem2() -> anyhow::Result<()> {
    let path = Path::new("input_day10.txt");
    let display = path.display();

    let file = match File::open(&path) {
        Err(why) => panic!("failed to open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut numbers = io::BufReader::new(file)
        .lines()
        .collect::<Result<Vec<_>, _>>()?
        .iter()
        .map(|x| x.parse::<u64>())
        .collect::<Result<Vec<_>, _>>()?;
    numbers.push(0);
    numbers.sort_unstable();
    numbers.push(numbers.last().unwrap() + 3);

    let differences = numbers
        .windows(2)
        .into_iter()
        .map(|xs| xs[1] - xs[0])
        .collect::<Vec<_>>();

    assert!(!differences.contains(&2));
    let split_by_threes = differences
        .as_slice()
        .split(|&x| x == 3)
        .collect::<Vec<_>>();

    let arrangements_per_group = split_by_threes
        .iter()
        .map(|xs| xs.iter().count())
        .map(arrangements)
        .collect::<Vec<_>>();

    let total_arrangements: u128 = arrangements_per_group.iter().map(|&x| x as u128).product();
    println!("{}", total_arrangements);

    Ok(())
}

fn arrangements(n: usize) -> usize {
    // TODO: Find a closed form that represents these values.
    match n {
        0 => 1,
        1 => 1,
        2 => 2,
        3 => 4,
        4 => 7,
        _ => panic!(),
    }
}
