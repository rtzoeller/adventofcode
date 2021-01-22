use anyhow::Result;
use std::convert::TryFrom;
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

#[derive(Debug)]
enum Orientation {
    North,
    East,
    South,
    West,
}

#[derive(Debug)]
struct Ship {
    pos_x: i32,
    pos_y: i32,
    dir: Orientation,
    waypoint_x: i32,
    waypoint_y: i32,
}

impl Default for Ship {
    fn default() -> Self {
        Self {
            pos_x: 0,
            pos_y: 0,
            dir: Orientation::East,
            waypoint_x: 10,
            waypoint_y: 1,
        }
    }
}

impl Ship {
    fn north(&mut self, magnitude: u32) {
        self.pos_y += i32::try_from(magnitude).unwrap();
    }

    fn east(&mut self, magnitude: u32) {
        self.pos_x += i32::try_from(magnitude).unwrap();
    }

    fn south(&mut self, magnitude: u32) {
        self.pos_y -= i32::try_from(magnitude).unwrap();
    }

    fn west(&mut self, magnitude: u32) {
        self.pos_x -= i32::try_from(magnitude).unwrap();
    }

    fn left(&mut self, magnitude: u32) {
        let turns = magnitude / 90;
        for _ in 0..turns {
            self.dir = match self.dir {
                Orientation::North => Orientation::West,
                Orientation::East => Orientation::North,
                Orientation::South => Orientation::East,
                Orientation::West => Orientation::South,
            }
        }
    }

    fn right(&mut self, magnitude: u32) {
        let turns = magnitude / 90;
        for _ in 0..turns {
            self.dir = match self.dir {
                Orientation::North => Orientation::East,
                Orientation::East => Orientation::South,
                Orientation::South => Orientation::West,
                Orientation::West => Orientation::North,
            }
        }
    }

    fn forward(&mut self, magnitude: u32) {
        match self.dir {
            Orientation::North => self.north(magnitude),
            Orientation::East => self.east(magnitude),
            Orientation::South => self.south(magnitude),
            Orientation::West => self.west(magnitude),
        }
    }

    fn waypoint_north(&mut self, magnitude: u32) {
        self.waypoint_y += i32::try_from(magnitude).unwrap();
    }

    fn waypoint_east(&mut self, magnitude: u32) {
        self.waypoint_x += i32::try_from(magnitude).unwrap();
    }

    fn waypoint_south(&mut self, magnitude: u32) {
        self.waypoint_y -= i32::try_from(magnitude).unwrap();
    }

    fn waypoint_west(&mut self, magnitude: u32) {
        self.waypoint_x -= i32::try_from(magnitude).unwrap();
    }

    fn waypoint_left(&mut self, magnitude: u32) {
        let turns = magnitude / 90;
        for _ in 0..turns {
            let x = self.waypoint_x;
            let y = self.waypoint_y;
            self.waypoint_x = -y;
            self.waypoint_y = x;
        }
    }

    fn waypoint_right(&mut self, magnitude: u32) {
        let turns = magnitude / 90;
        for _ in 0..turns {
            let x = self.waypoint_x;
            let y = self.waypoint_y;
            self.waypoint_x = y;
            self.waypoint_y = -x;
        }
    }

    fn forward_towards_waypoint(&mut self, magnitude: u32) {
        let signed_magnitude = i32::try_from(magnitude).unwrap();
        self.pos_x += self.waypoint_x * signed_magnitude;
        self.pos_y += self.waypoint_y * signed_magnitude;
    }
}

pub fn problem1() -> anyhow::Result<()> {
    let path = Path::new("input_day12.txt");
    let display = path.display();

    let file = match File::open(&path) {
        Err(why) => panic!("failed to open {}: {}", display, why),
        Ok(file) => file,
    };

    let lines = io::BufReader::new(file)
        .lines()
        .collect::<Result<Vec<_>, _>>()?;

    let mut ship = Ship::default();

    for line in &lines {
        if line.len() < 2 {
            return Err(ProblemInvariantError.into());
        }

        let command = line.chars().next().unwrap();
        let magnitude = line[1..].parse::<u32>()?;

        match command {
            'N' => ship.north(magnitude),
            'E' => ship.east(magnitude),
            'S' => ship.south(magnitude),
            'W' => ship.west(magnitude),
            'L' => ship.left(magnitude),
            'R' => ship.right(magnitude),
            'F' => ship.forward(magnitude),
            _ => return Err(ProblemInvariantError.into()),
        }
    }

    println!("{}", ship.pos_x.abs() + ship.pos_y.abs());

    Ok(())
}

pub fn problem2() -> anyhow::Result<()> {
    let path = Path::new("input_day12.txt");
    let display = path.display();

    let file = match File::open(&path) {
        Err(why) => panic!("failed to open {}: {}", display, why),
        Ok(file) => file,
    };

    let lines = io::BufReader::new(file)
        .lines()
        .collect::<Result<Vec<_>, _>>()?;

    let mut ship = Ship::default();

    for line in &lines {
        if line.len() < 2 {
            return Err(ProblemInvariantError.into());
        }

        let command = line.chars().next().unwrap();
        let magnitude = line[1..].parse::<u32>()?;

        match command {
            'N' => ship.waypoint_north(magnitude),
            'E' => ship.waypoint_east(magnitude),
            'S' => ship.waypoint_south(magnitude),
            'W' => ship.waypoint_west(magnitude),
            'L' => ship.waypoint_left(magnitude),
            'R' => ship.waypoint_right(magnitude),
            'F' => ship.forward_towards_waypoint(magnitude),
            _ => return Err(ProblemInvariantError.into()),
        }
    }

    println!("{}", ship.pos_x.abs() + ship.pos_y.abs());

    Ok(())
}
