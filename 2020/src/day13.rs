use anyhow::Result;
use regex::Regex;
use std::convert::identity;
use std::fs::File;
use std::io::{self, prelude::*};
use std::path::Path;

use lazy_static::lazy_static;

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
    let path = Path::new("input_day13.txt");
    let display = path.display();

    let file = match File::open(&path) {
        Err(why) => panic!("failed to open {}: {}", display, why),
        Ok(file) => file,
    };

    let lines = io::BufReader::new(file)
        .lines()
        .collect::<Result<Vec<_>, _>>()?;

    lazy_static! {
        static ref RE: Regex = Regex::new(r"(\d+|x),?").unwrap();
    }
    let start_time = lines[0].parse::<u32>()?;
    let captures = RE.captures_iter(&lines[1]);
    let all_busses = captures
        .map(|capture| match capture.get(1).unwrap().as_str() {
            "x" => None,
            s => Some(s.parse::<u32>().unwrap()),
        })
        .collect::<Vec<_>>();

    let active_busses = all_busses.iter().filter_map(|x| *x).collect::<Vec<_>>();

    let previous_departures = active_busses.iter().map(|t| ((start_time - 1) / t) * t);
    let next_arrivals = previous_departures
        .zip(&active_busses)
        .map(|(t0, dt)| t0 + dt)
        .collect::<Vec<_>>();
    let (soonest_arrival_index, soonest_arrival_time) = next_arrivals
        .iter()
        .enumerate()
        .min_by_key(|&(_, x)| x)
        .unwrap();
    let soonest_arrival_id = active_busses[soonest_arrival_index];

    println!(
        "{}",
        soonest_arrival_id * (soonest_arrival_time - start_time)
    );

    Ok(())
}

pub fn problem2() -> anyhow::Result<()> {
    let path = Path::new("input_day13.txt");
    let display = path.display();

    let file = match File::open(&path) {
        Err(why) => panic!("failed to open {}: {}", display, why),
        Ok(file) => file,
    };

    let lines = io::BufReader::new(file)
        .lines()
        .collect::<Result<Vec<_>, _>>()?;

    lazy_static! {
        static ref RE: Regex = Regex::new(r"(\d+|x),?").unwrap();
    }
    let captures = RE.captures_iter(&lines[1]);
    let all_busses_and_offsets = captures
        .map(|capture| match capture.get(1).unwrap().as_str() {
            "x" => None,
            s => Some(s.parse::<u32>().unwrap()),
        })
        .enumerate()
        .collect::<Vec<_>>();

    let active_busses_and_offsets = all_busses_and_offsets
        .iter()
        .map(|(offset, bus)| match bus {
            None => None,
            Some(id) => Some((offset, id)),
        })
        .filter_map(identity)
        .collect::<Vec<_>>();

    let mut affine = (0, 1);
    for (&offset, &id) in &active_busses_and_offsets {
        affine = foo(&affine, id as u128, offset as u128);
    }

    println!("{}", affine.0);

    Ok(())
}

/// Given an affine function which produces integer multiples of some values and offsets,
/// compute a new affine function which also includes a new divisor at some offset.
///
/// `affine_function` - (a, b) representing y = a + b * x
fn foo(affine_function: &(u128, u128), divisor: u128, offset: u128) -> (u128, u128) {
    let (a, b) = *affine_function;
    let mut i = a;
    let mut result = (None, None);

    loop {
        if (i + offset) % divisor == 0 && (i + offset) != 0 {
            match result.0 {
                None => result.0 = Some(i),
                Some(_) => {
                    result.1 = Some(i - result.0.unwrap());
                    break;
                }
            }
        }
        i += b;
    }

    (result.0.unwrap(), result.1.unwrap())
}
