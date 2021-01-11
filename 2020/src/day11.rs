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

#[derive(Debug, Copy, Clone, PartialEq)]
enum Position {
    Floor,
    Empty,
    Occupied,
}

#[derive(Debug, PartialEq)]
struct Ferry {
    seats: Vec<Vec<Position>>,
}

impl Ferry {
    fn new(lines: &[&str]) -> Ferry {
        Ferry {
            seats: lines
                .iter()
                .map(|row| {
                    row.chars()
                        .map(|c| match c {
                            '.' => Position::Floor,
                            'L' => Position::Empty,
                            '#' => Position::Occupied,
                            _ => panic!("Unexpected character."),
                        })
                        .collect()
                })
                .collect(),
        }
    }

    fn next1(&self) -> Ferry {
        let mut rows = Vec::with_capacity(self.seats.len());
        for y in 0..self.seats.len() {
            let mut row = Vec::with_capacity(self.seats[y].len());
            for x in 0..self.seats[y].len() {
                let neighbors = self
                    .neighbors1(x, y)
                    .into_iter()
                    .filter(|x| *x == Some(&Position::Occupied))
                    .count();
                let new_seat = match self.seats[y][x] {
                    Position::Empty => {
                        if neighbors == 0 {
                            Position::Occupied
                        } else {
                            self.seats[y][x]
                        }
                    }
                    Position::Occupied => {
                        if neighbors >= 4 {
                            Position::Empty
                        } else {
                            self.seats[y][x]
                        }
                    }
                    Position::Floor => Position::Floor,
                };
                row.push(new_seat);
            }
            rows.push(row);
        }
        Ferry { seats: rows }
    }

    fn next2(&self) -> Ferry {
        let mut rows = Vec::with_capacity(self.seats.len());
        for y in 0..self.seats.len() {
            let mut row = Vec::with_capacity(self.seats[y].len());
            for x in 0..self.seats[y].len() {
                let neighbors = self
                    .neighbors2(x, y)
                    .into_iter()
                    .filter(|x| *x == Some(&Position::Occupied))
                    .count();
                let new_seat = match self.seats[y][x] {
                    Position::Empty => {
                        if neighbors == 0 {
                            Position::Occupied
                        } else {
                            self.seats[y][x]
                        }
                    }
                    Position::Occupied => {
                        if neighbors >= 5 {
                            Position::Empty
                        } else {
                            self.seats[y][x]
                        }
                    }
                    Position::Floor => Position::Floor,
                };
                row.push(new_seat);
            }
            rows.push(row);
        }
        Ferry { seats: rows }
    }

    fn neighbors1(&self, x: usize, y: usize) -> Vec<Option<&Position>> {
        let mut result = Vec::with_capacity(8);
        result.push(get_cell(&self.seats, x, y, -1, -1));
        result.push(get_cell(&self.seats, x, y, -1, 0));
        result.push(get_cell(&self.seats, x, y, -1, 1));
        result.push(get_cell(&self.seats, x, y, 0, -1));
        result.push(get_cell(&self.seats, x, y, 0, 1));
        result.push(get_cell(&self.seats, x, y, 1, -1));
        result.push(get_cell(&self.seats, x, y, 1, 0));
        result.push(get_cell(&self.seats, x, y, 1, 1));
        result
    }

    fn neighbors2(&self, x: usize, y: usize) -> Vec<Option<&Position>> {
        let mut result = Vec::with_capacity(8);
        result.push(get_non_floor_cell_in_direction(&self.seats, x, y, -1, -1));
        result.push(get_non_floor_cell_in_direction(&self.seats, x, y, -1, 0));
        result.push(get_non_floor_cell_in_direction(&self.seats, x, y, -1, 1));
        result.push(get_non_floor_cell_in_direction(&self.seats, x, y, 0, -1));
        result.push(get_non_floor_cell_in_direction(&self.seats, x, y, 0, 1));
        result.push(get_non_floor_cell_in_direction(&self.seats, x, y, 1, -1));
        result.push(get_non_floor_cell_in_direction(&self.seats, x, y, 1, 0));
        result.push(get_non_floor_cell_in_direction(&self.seats, x, y, 1, 1));
        result
    }
}

#[allow(clippy::ptr_arg)]
fn get_cell(
    seats: &Vec<Vec<Position>>,
    x: usize,
    y: usize,
    x_offset: isize,
    y_offset: isize,
) -> Option<&Position> {
    let neighbor_x = (x as isize) + x_offset;
    let neighbor_y = (y as isize) + y_offset;
    if neighbor_y < 0 || neighbor_y >= seats.len() as isize {
        return None;
    }
    if neighbor_x < 0 || neighbor_x >= seats[neighbor_y as usize].len() as isize {
        return None;
    }
    Some(&seats[neighbor_y as usize][neighbor_x as usize])
}

#[allow(clippy::ptr_arg)]
fn get_non_floor_cell_in_direction(
    seats: &Vec<Vec<Position>>,
    x: usize,
    y: usize,
    x_offset: isize,
    y_offset: isize,
) -> Option<&Position> {
    let mut n = 1;
    loop {
        let seat = get_cell(seats, x, y, x_offset * n, y_offset * n);
        if seat != Some(&Position::Floor) {
            return seat;
        } else {
            n += 1;
        }
    }
}

pub fn problem1() -> anyhow::Result<()> {
    let path = Path::new("input_day11.txt");
    let display = path.display();

    let file = match File::open(&path) {
        Err(why) => panic!("failed to open {}: {}", display, why),
        Ok(file) => file,
    };

    let lines = io::BufReader::new(file)
        .lines()
        .collect::<Result<Vec<_>, _>>()?;

    let mut ferry = Ferry::new(&lines.iter().map(AsRef::as_ref).collect::<Vec<_>>());
    loop {
        let next = ferry.next1();
        if ferry == next {
            break;
        }
        ferry = next;
    }

    let occupied_seats: usize = ferry
        .seats
        .iter()
        .map(|row| row.iter().filter(|s| **s == Position::Occupied).count())
        .sum();

    println!("{}", occupied_seats);

    Ok(())
}

pub fn problem2() -> anyhow::Result<()> {
    let path = Path::new("input_day11.txt");
    let display = path.display();

    let file = match File::open(&path) {
        Err(why) => panic!("failed to open {}: {}", display, why),
        Ok(file) => file,
    };

    let lines = io::BufReader::new(file)
        .lines()
        .collect::<Result<Vec<_>, _>>()?;

    let mut ferry = Ferry::new(&lines.iter().map(AsRef::as_ref).collect::<Vec<_>>());
    loop {
        let next = ferry.next2();
        if ferry == next {
            break;
        }
        ferry = next;
    }

    let occupied_seats: usize = ferry
        .seats
        .iter()
        .map(|row| row.iter().filter(|s| **s == Position::Occupied).count())
        .sum();

    println!("{}", occupied_seats);

    Ok(())
}
