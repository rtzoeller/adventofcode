#[macro_use] extern crate lazy_static;
extern crate regex;

mod day1;
mod day2;
mod day3;

fn main() -> anyhow::Result<()> {
    day1::problem1()?;
    day1::problem2()?;
    day2::problem1()?;
    day2::problem2()?;
    day3::problem1()?;
    day3::problem2()?;
    Ok(())
}
