use anyhow::Result;
use lazy_static::lazy_static;
use regex::Regex;
use std::fs::File;
use std::io::{self, prelude::*};
use std::path::Path;

#[derive(Debug)]
struct ParseInstructionError;

impl std::error::Error for ParseInstructionError {}

impl std::fmt::Display for ParseInstructionError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Failed to parse instruction.")
    }
}

#[derive(Debug)]
struct ProcessorStateError;

impl std::error::Error for ProcessorStateError {}

impl std::fmt::Display for ProcessorStateError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Illegal processor state encountered.")
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Instruction {
    Acc(isize),
    Jmp(isize),
    Nop(isize),
}

#[derive(Debug)]
struct ProcessorState {
    visit_counts: Vec<u32>,
    instruction_counter: usize,
    accumulator: isize,
}

#[derive(Debug, PartialEq)]
enum TerminationMode {
    Normal,
    Deadlock,
}

pub fn problem1() -> anyhow::Result<()> {
    let path = Path::new("input_day8.txt");
    let display = path.display();

    let file = match File::open(&path) {
        Err(why) => panic!("failed to open {}: {}", display, why),
        Ok(file) => file,
    };

    let lines = io::BufReader::new(file)
        .lines()
        .collect::<Result<Vec<_>, _>>()?;

    let instructions = lines
        .iter()
        .map(|s| parse_instruction(s))
        .collect::<Option<Vec<_>>>()
        .ok_or(ParseInstructionError)?;

    let mut processor_state = ProcessorState {
        visit_counts: vec![0; instructions.len()],
        instruction_counter: 0,
        accumulator: 0,
    };

    let termination_mode = tick_until_done_or_deadlocked(&instructions, &mut processor_state)?;
    assert_eq!(TerminationMode::Deadlock, termination_mode);
    println!("{}", processor_state.accumulator);

    Ok(())
}

pub fn problem2() -> anyhow::Result<()> {
    let path = Path::new("input_day8.txt");
    let display = path.display();

    let file = match File::open(&path) {
        Err(why) => panic!("failed to open {}: {}", display, why),
        Ok(file) => file,
    };

    let lines = io::BufReader::new(file)
        .lines()
        .collect::<Result<Vec<_>, _>>()?;

    let instructions = lines
        .iter()
        .map(|s| parse_instruction(s))
        .collect::<Option<Vec<_>>>()
        .ok_or(ParseInstructionError)?;

    let mut processor_state = ProcessorState {
        visit_counts: vec![0; instructions.len()],
        instruction_counter: 0,
        accumulator: 0,
    };
    let mut found = false;

    for i in 0..instructions.len() {
        let mut mutated_instructions = instructions.to_vec();
        if let Instruction::Jmp(x) = mutated_instructions[i] {
            mutated_instructions[i] = Instruction::Nop(x);
        } else if let Instruction::Nop(x) = mutated_instructions[i] {
            mutated_instructions[i] = Instruction::Jmp(x);
        } else {
            continue;
        }

        processor_state = ProcessorState {
            visit_counts: vec![0; instructions.len()],
            instruction_counter: 0,
            accumulator: 0,
        };

        let termination_mode =
            tick_until_done_or_deadlocked(&mutated_instructions, &mut processor_state)?;
        if termination_mode == TerminationMode::Normal {
            found = true;
            break;
        }
    }

    assert!(found);
    println!("{}", processor_state.accumulator);

    Ok(())
}

fn parse_instruction(line: &str) -> Option<Instruction> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^(acc|jmp|nop) ([+-]\d+)$").unwrap();
    }

    let captures = RE.captures(line)?;
    let instruction = match captures.get(1)?.as_str() {
        "acc" => Instruction::Acc(captures.get(2)?.as_str().parse::<isize>().ok()?),
        "jmp" => Instruction::Jmp(captures.get(2)?.as_str().parse::<isize>().ok()?),
        "nop" => Instruction::Nop(captures.get(2)?.as_str().parse::<isize>().ok()?),
        _ => return None,
    };

    Some(instruction)
}

fn checked_unsigned_signed_add(x: usize, y: isize) -> Option<usize> {
    if y.is_negative() {
        x.checked_sub(y.wrapping_abs() as usize)
    } else {
        x.checked_add(y as usize)
    }
}

fn tick(instructions: &[Instruction], processor_state: &mut ProcessorState) -> anyhow::Result<()> {
    processor_state.visit_counts[processor_state.instruction_counter] += 1;
    match &instructions[(processor_state.instruction_counter)] {
        Instruction::Acc(value) => {
            processor_state.instruction_counter = processor_state
                .instruction_counter
                .checked_add(1)
                .ok_or(ProcessorStateError)?;
            processor_state.accumulator = processor_state
                .accumulator
                .checked_add(*value)
                .ok_or(ProcessorStateError)?;
        }
        Instruction::Jmp(value) => {
            processor_state.instruction_counter =
                checked_unsigned_signed_add(processor_state.instruction_counter, *value)
                    .ok_or(ProcessorStateError)?;
        }
        Instruction::Nop(_) => {
            processor_state.instruction_counter = processor_state
                .instruction_counter
                .checked_add(1)
                .ok_or(ProcessorStateError)?
        }
    }

    Ok(())
}

fn tick_until_done_or_deadlocked(
    instructions: &[Instruction],
    processor_state: &mut ProcessorState,
) -> anyhow::Result<TerminationMode> {
    while processor_state.instruction_counter != instructions.len()
        && processor_state.visit_counts[processor_state.instruction_counter] != 1
    {
        tick(&instructions, processor_state)?;
    }

    if processor_state.instruction_counter == instructions.len() {
        Ok(TerminationMode::Normal)
    } else if processor_state.visit_counts[processor_state.instruction_counter] == 1 {
        Ok(TerminationMode::Deadlock)
    } else {
        panic!("tick_until_done_or_deadlocked() entered an illegal state.");
    }
}
