use anyhow::Result;
use regex::Regex;
use std::convert::TryInto;
use std::fs::File;
use std::io::{self, prelude::*};
use std::path::Path;

struct BoardingPass {
    row: [char; 7],
    seat: [char; 3],
}

pub fn problem1() -> anyhow::Result<()> {
    let path = Path::new("input_day5.txt");
    let display = path.display();

    let file = match File::open(&path) {
        Err(why) => panic!("failed to open {}: {}", display, why),
        Ok(file) => file,
    };

    let lines = io::BufReader::new(file)
        .lines()
        .collect::<Result<Vec<_>, _>>()?;

    let boarding_passes = parse_boarding_passes(&lines)?;
    let seat_numbers = boarding_passes
        .iter()
        .map(compute_seat_number)
        .collect::<Result<Vec<u32>>>()?;
    let max_seat_number = seat_numbers.iter().max().unwrap();
    println!("{}", max_seat_number);
    Ok(())
}

pub fn problem2() -> anyhow::Result<()> {
    let path = Path::new("input_day5.txt");
    let display = path.display();

    let file = match File::open(&path) {
        Err(why) => panic!("failed to open {}: {}", display, why),
        Ok(file) => file,
    };

    let lines = io::BufReader::new(file)
        .lines()
        .collect::<Result<Vec<_>, _>>()?;

    let boarding_passes = parse_boarding_passes(&lines)?;
    let mut seat_numbers = boarding_passes
        .iter()
        .map(compute_seat_number)
        .collect::<Result<Vec<u32>>>()?;
    seat_numbers.sort_unstable();

    let empty_seat = seat_numbers
        .iter()
        .zip(seat_numbers[1..].iter())
        .find(|(x, y)| *y - *x != 1)
        .unwrap()
        .0
        + 1;
    println!("{}", empty_seat);
    Ok(())
}

fn parse_boarding_passes(lines: &[String]) -> anyhow::Result<Vec<BoardingPass>> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^([FB]{7})([LR]{3})$").unwrap();
    }

    Ok(lines
        .iter()
        .map(|line| RE.captures(&line).unwrap()) // TODO: Remove unwrap and return error
        .map(|groups| BoardingPass {
            row: groups
                .get(1)
                .unwrap()
                .as_str()
                .chars()
                .collect::<Vec<char>>()
                .try_into()
                .unwrap(),
            seat: groups
                .get(2)
                .unwrap()
                .as_str()
                .chars()
                .collect::<Vec<char>>()
                .try_into()
                .unwrap(),
        })
        .collect())
}

fn compute_seat_number(boarding_pass: &BoardingPass) -> anyhow::Result<u32> {
    let mut row_bounds = (0, 127);
    for c in boarding_pass.row.iter() {
        row_bounds = binary_search_row(*c, row_bounds);
    }
    assert_eq!(row_bounds.0, row_bounds.1);

    let mut seat_bounds = (0, 7);
    for c in boarding_pass.seat.iter() {
        seat_bounds = binary_search_seat(*c, seat_bounds);
    }
    assert_eq!(seat_bounds.0, seat_bounds.1);

    Ok(row_bounds.0 * 8 + seat_bounds.0)
}

fn binary_search_row(c: char, (lower, upper): (u32, u32)) -> (u32, u32) {
    match c {
        'F' => (lower, lower + ((upper - lower) / 2)),
        'B' => (lower + ((upper - lower + 1) / 2), upper),
        _ => panic!("Unexpected character {}.", c),
    }
}

fn binary_search_seat(c: char, (lower, upper): (u32, u32)) -> (u32, u32) {
    match c {
        'L' => (lower, lower + ((upper - lower) / 2)),
        'R' => (lower + ((upper - lower + 1) / 2), upper),
        _ => panic!("Unexpected character {}.", c),
    }
}
