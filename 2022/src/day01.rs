use crate::file_wrappers::get_lines_from_embedded_file;

use std::cmp::Reverse;

#[derive(Debug)]
pub struct Elf {
    snacks: Vec<u32>,
}

fn get_elves() -> anyhow::Result<Vec<Elf>> {
    let lines = get_lines_from_embedded_file("input_day01.txt")?;
    let numbers: Vec<_> = lines.iter().map(|s| str::parse::<u32>(s).ok()).collect();
    let elves: Vec<_> = numbers
        .split(|x| x.is_none())
        .map(|xs| Elf {
            snacks: xs.iter().map(|x| x.unwrap()).collect(),
        })
        .collect();
    Ok(elves)
}

fn total_snacks(elf: &Elf) -> u32 {
    elf.snacks.iter().sum()
}

pub fn problem1() -> anyhow::Result<u32> {
    let elves = get_elves()?;

    let max_snacks = elves
        .iter()
        .map(total_snacks)
        .max()
        .expect("There must be at least one elf.");
    Ok(max_snacks)
}

pub fn problem2() -> anyhow::Result<u32> {
    let mut elves = get_elves()?;
    elves.sort_by_key(|e| Reverse(total_snacks(e)));
    let top_three = elves.iter().take(3);
    let top_three_snacks: u32 = top_three.map(total_snacks).sum();
    Ok(top_three_snacks)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem1() {
        assert_eq!(problem1().unwrap(), 70374);
    }

    #[test]
    fn test_problem2() {
        assert_eq!(problem2().unwrap(), 204610);
    }
}
