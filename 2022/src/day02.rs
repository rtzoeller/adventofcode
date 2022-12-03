use std::{collections::HashMap, hash::Hash};

use lazy_static::lazy_static;

use crate::file_wrappers::get_lines_from_embedded_file;

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
enum RPSMove {
    Rock,
    Paper,
    Scissors,
}

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
enum RPSResult {
    Win,
    Draw,
    Loss,
}

lazy_static! {
    static ref RPS_WIN_LOSS_MAP: HashMap<(RPSMove, RPSMove), RPSResult> = {
        let mut m = HashMap::new();
        m.insert((RPSMove::Rock, RPSMove::Rock), RPSResult::Draw);
        m.insert((RPSMove::Rock, RPSMove::Paper), RPSResult::Loss);
        m.insert((RPSMove::Rock, RPSMove::Scissors), RPSResult::Win);
        m.insert((RPSMove::Paper, RPSMove::Rock), RPSResult::Win);
        m.insert((RPSMove::Paper, RPSMove::Paper), RPSResult::Draw);
        m.insert((RPSMove::Paper, RPSMove::Scissors), RPSResult::Loss);
        m.insert((RPSMove::Scissors, RPSMove::Rock), RPSResult::Loss);
        m.insert((RPSMove::Scissors, RPSMove::Paper), RPSResult::Win);
        m.insert((RPSMove::Scissors, RPSMove::Scissors), RPSResult::Draw);
        m
    };
}

fn score(own_move: RPSMove, opponent_move: RPSMove) -> u32 {
    let win_points = match RPS_WIN_LOSS_MAP.get(&(own_move, opponent_move)).unwrap() {
        RPSResult::Win => 6u32,
        RPSResult::Draw => 3u32,
        RPSResult::Loss => 0u32,
    };

    let move_points = match own_move {
        RPSMove::Rock => 1u32,
        RPSMove::Paper => 2u32,
        RPSMove::Scissors => 3u32,
    };

    win_points + move_points
}

fn get_strategy_guide() -> anyhow::Result<Vec<(char, char)>> {
    let lines = get_lines_from_embedded_file("input_day02.txt")?;
    let chars: Vec<_> = lines
        .iter()
        .filter_map(|s| {
            if let [a, b, ..] = s.split_whitespace().take(2).collect::<Vec<_>>().as_slice() {
                Some((a.chars().next().unwrap(), b.chars().next().unwrap()))
            } else {
                None
            }
        })
        .collect();

    Ok(chars)
}

fn get_moves() -> anyhow::Result<Vec<(RPSMove, RPSMove)>> {
    let strategy_guide = get_strategy_guide()?;
    let moves: Vec<_> = strategy_guide
        .iter()
        .map(|(opponent, own)| {
            (
                match opponent {
                    'A' => RPSMove::Rock,
                    'B' => RPSMove::Paper,
                    'C' => RPSMove::Scissors,
                    c => panic!("Unrecognized move {}", c),
                },
                match own {
                    'X' => RPSMove::Rock,
                    'Y' => RPSMove::Paper,
                    'Z' => RPSMove::Scissors,
                    c => panic!("Unrecognized move {}", c),
                },
            )
        })
        .collect();

    Ok(moves)
}

fn get_move_and_result() -> anyhow::Result<Vec<(RPSMove, RPSResult)>> {
    let strategy_guide = get_strategy_guide()?;
    let moves: Vec<_> = strategy_guide
        .iter()
        .map(|(opponent, result)| {
            (
                match opponent {
                    'A' => RPSMove::Rock,
                    'B' => RPSMove::Paper,
                    'C' => RPSMove::Scissors,
                    c => panic!("Unrecognized move {}", c),
                },
                match result {
                    'X' => RPSResult::Loss,
                    'Y' => RPSResult::Draw,
                    'Z' => RPSResult::Win,
                    c => panic!("Unrecognized result {}", c),
                },
            )
        })
        .collect();

    Ok(moves)
}

pub fn problem1() -> anyhow::Result<u32> {
    let moves = get_moves()?;
    let score = moves
        .iter()
        .map(|(opponent, own)| score(*own, *opponent))
        .sum();
    Ok(score)
}

pub fn problem2() -> anyhow::Result<u32> {
    let mut necessary_move_map = HashMap::<(RPSMove, RPSResult), RPSMove>::new();
    let possible_moves = [RPSMove::Rock, RPSMove::Paper, RPSMove::Scissors];
    for own in possible_moves {
        for opponent in possible_moves {
            let result = RPS_WIN_LOSS_MAP.get(&(own, opponent)).unwrap();
            necessary_move_map.insert((opponent, *result), own);
        }
    }

    let moves = get_move_and_result()?;
    let score = moves
        .iter()
        .map(|(opponent, result)| {
            let own = necessary_move_map.get(&(*opponent, *result)).unwrap();
            score(*own, *opponent)
        })
        .sum();
    Ok(score)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem1() {
        assert_eq!(problem1().unwrap(), 15632);
    }

    #[test]
    fn test_problem2() {
        assert_eq!(problem2().unwrap(), 14416);
    }
}
