use anyhow::Result;
use std::fs::File;
use std::io::{self, prelude::*};
use std::path::Path;

pub fn problem1() -> anyhow::Result<()> {
    let num_trees = count_trees(3, 1)?;
    println!("{}", num_trees);
    Ok(())
}

pub fn problem2() -> anyhow::Result<()> {
    let product = count_trees(1, 1)?
        * count_trees(3, 1)?
        * count_trees(5, 1)?
        * count_trees(7, 1)?
        * count_trees(1, 2)?;
    println!("{}", product);
    Ok(())
}

fn count_trees(x_slope: usize, y_slope: usize) -> anyhow::Result<u32> {
    let path = Path::new("input_day3.txt");
    let display = path.display();

    let file = match File::open(&path) {
        Err(why) => panic!("failed to open {}: {}", display, why),
        Ok(file) => file,
    };

    let grid = io::BufReader::new(file)
        .lines()
        .collect::<Result<Vec<_>, _>>()?
        .iter()
        .map(|s| s.as_str())
        .map(parse_line)
        .collect::<Vec<Vec<bool>>>();

    let height: usize = grid.len();
    let width: usize = grid[0].len();
    let mut num_trees: u32 = 0;
    let mut x: usize = 0;
    let mut y: usize = 0;

    while y < height {
        if grid[y][x % width] {
            num_trees += 1;
        }

        x += x_slope;
        y += y_slope;
    }

    Ok(num_trees)
}

fn parse_line(s: &str) -> Vec<bool> {
    s.chars()
        .map(|c| match c {
            '.' => false,
            '#' => true,
            _ => panic!("Unexpected character"),
        })
        .collect::<Vec<bool>>()
}
