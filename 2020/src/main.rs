#[macro_use] extern crate lazy_static;
extern crate regex;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

fn main() -> anyhow::Result<()> {
    day1::problem1()?;
    day1::problem2()?;
    day2::problem1()?;
    day2::problem2()?;
    day3::problem1()?;
    day3::problem2()?;
    day4::problem1()?;
    day4::problem2()?;
    day5::problem1()?;
    day5::problem2()?;
    Ok(())
}
