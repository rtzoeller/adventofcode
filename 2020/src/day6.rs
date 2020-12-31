use anyhow::Result;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, prelude::*};
use std::path::Path;

pub fn problem1() -> anyhow::Result<()> {
    let path = Path::new("input_day6.txt");
    let display = path.display();

    let file = match File::open(&path) {
        Err(why) => panic!("failed to open {}: {}", display, why),
        Ok(file) => file,
    };

    let lines = io::BufReader::new(file)
        .lines()
        .collect::<Result<Vec<_>, _>>()?;

    let forms = problem1_parse_forms(&lines);
    let sum: usize = forms.iter().map(|f| f.len()).sum();
    println!("{}", sum);
    Ok(())
}

pub fn problem2() -> anyhow::Result<()> {
    let path = Path::new("input_day6.txt");
    let display = path.display();

    let file = match File::open(&path) {
        Err(why) => panic!("failed to open {}: {}", display, why),
        Ok(file) => file,
    };

    let lines = io::BufReader::new(file)
        .lines()
        .collect::<Result<Vec<_>, _>>()?;

    let forms = problem2_parse_forms(&lines);
    let sum: usize = forms.iter().map(|f| f.len()).sum();
    println!("{}", sum);
    Ok(())
}

// TODO: These ought to take &[&str]
fn problem1_parse_forms(lines: &[String]) -> Vec<HashSet<char>> {
    let groups = lines.split(|line| line.is_empty());
    groups.map(problem1_parse_form).collect()
}

fn problem1_parse_form(group: &[String]) -> HashSet<char> {
    group
        .iter()
        .map(|line| line.chars())
        .fold(HashSet::<char>::new(), |set, chars| {
            set.union(&chars.collect()).copied().collect()
        })
}

fn problem2_parse_forms(lines: &[String]) -> Vec<HashSet<char>> {
    let groups = lines.split(|line| line.is_empty());
    groups.map(problem2_parse_form).collect()
}

fn problem2_parse_form(group: &[String]) -> HashSet<char> {
    let chars_by_line = group.iter().map(|line| line.chars());

    let mut set = HashSet::<char>::new();
    for (i, chars) in chars_by_line.enumerate() {
        if i == 0 {
            set = chars.collect();
        } else {
            set = set.intersection(&chars.collect()).copied().collect();
        }
    }

    set
}
