use anyhow::Result;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::convert::TryInto;
use std::fs::File;
use std::io::{self, prelude::*};
use std::path::Path;

#[derive(Debug)]
struct BagRule<'a> {
    name: &'a str,
    possible_contents: Vec<(u32, &'a str)>,
}

pub fn problem1() -> anyhow::Result<()> {
    let path = Path::new("input_day7.txt");
    let display = path.display();

    let file = match File::open(&path) {
        Err(why) => panic!("failed to open {}: {}", display, why),
        Ok(file) => file,
    };

    let lines = io::BufReader::new(file)
        .lines()
        .collect::<Result<Vec<_>, _>>()?;

    let bag_rules = parse_bag_rules(&lines.iter().map(AsRef::as_ref).collect::<Vec<_>>())?;
    let possible_direct_containers = compute_possible_direct_containers(&bag_rules)?;

    let initial_color = "shiny gold";
    let mut to_visit = vec![initial_color];
    let mut visited_colors = HashSet::<&str>::new();

    while !to_visit.is_empty() {
        let active = to_visit.pop().unwrap();
        visited_colors.insert(active);
        let empty = HashSet::<&str>::new();
        let reachable = possible_direct_containers.get(active).unwrap_or(&empty);
        let new_to_visit = reachable
            .iter()
            .filter(|s| !visited_colors.contains(s.to_owned()))
            .copied()
            .collect::<Vec<_>>();
        to_visit.extend(&new_to_visit);
    }
    visited_colors.remove(&initial_color);
    println!("{}", visited_colors.len());
    Ok(())
}

pub fn problem2() -> anyhow::Result<()> {
    let path = Path::new("input_day7.txt");
    let display = path.display();

    let file = match File::open(&path) {
        Err(why) => panic!("failed to open {}: {}", display, why),
        Ok(file) => file,
    };

    let lines = io::BufReader::new(file)
        .lines()
        .collect::<Result<Vec<_>, _>>()?;

    let bag_rules = parse_bag_rules(&lines.iter().map(AsRef::as_ref).collect::<Vec<_>>())?;
    let bag_rules_map = bag_rules
        .iter()
        .map(|br| (br.name, br))
        .collect::<HashMap<&str, &BagRule>>();

    let initial_color = "shiny gold";
    let mut to_visit = vec![(initial_color)];
    let mut num_visited = 0;

    while !to_visit.is_empty() {
        let active_name = to_visit.pop().unwrap();
        let active_bag_rule = bag_rules_map.get(active_name).unwrap();
        num_visited += 1;
        for new_to_visit in &active_bag_rule.possible_contents {
            to_visit.extend(
                &std::iter::repeat(new_to_visit.1)
                    .take(new_to_visit.0.try_into().unwrap())
                    .collect::<Vec<_>>(),
            );
        }
    }
    println!("{}", num_visited - 1);
    Ok(())
}

fn parse_bag_rules<'a>(lines: &[&'a str]) -> anyhow::Result<Vec<BagRule<'a>>> {
    lines.iter().map(|line| parse_bag_rule(&line)).collect()
}

fn parse_bag_rule(line: &str) -> anyhow::Result<BagRule> {
    #[rustfmt::skip]
    lazy_static! {
        static ref CONTAINER_BAG: Regex = Regex::new(r"^([a-z ]+) bags contain ").unwrap();
        static ref CONTAINED_BAG_NONZERO: Regex = Regex::new(r"(\d+) ([a-z ]+) bags?[,.]").unwrap();
        static ref CONTAINED_BAG_ZERO: Regex = #[allow(clippy::trivial_regex)] Regex::new(r"no other bags\.").unwrap();
    }

    // TODO: Handle errors
    let container_name = CONTAINER_BAG
        .captures(&line)
        .unwrap()
        .get(1)
        .unwrap()
        .as_str();

    let contained_bags = CONTAINED_BAG_NONZERO
        .captures_iter(&line)
        .map(|captures| {
            (
                captures.get(1).unwrap().as_str().parse::<u32>().unwrap(),
                captures.get(2).unwrap().as_str(),
            )
        })
        .collect::<Vec<_>>();

    // TODO: Validate the zero bag text matches

    Ok(BagRule {
        name: container_name,
        possible_contents: contained_bags,
    })
}

fn compute_possible_direct_containers<'a>(
    bag_rules: &'a [BagRule],
) -> anyhow::Result<HashMap<&'a str, HashSet<&'a str>>> {
    let mut possible_direct_containers = HashMap::<&str, HashSet<&str>>::new();
    for bag_rule in bag_rules {
        for child in &bag_rule.possible_contents {
            let (_, child_name) = child;
            match possible_direct_containers.get_mut(child_name) {
                None => {
                    possible_direct_containers
                        .insert(&child_name, vec![bag_rule.name].into_iter().collect());
                }
                Some(set) => {
                    set.insert(bag_rule.name);
                }
            };
        }
    }

    Ok(possible_direct_containers)
}
