use std::convert::TryFrom;
use std::error::Error;
use std::fs;

struct State {
    register_x: i32,
    cycle_num: i32,
}

enum Command {
    Noop,
    Addx(i32),
}

impl TryFrom<&str> for Command {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        if s == "noop" {
            return Ok(Command::Noop);
        } else if s.starts_with("addx") {
            let n = s
                .split_whitespace()
                .nth(1)
                .ok_or("No nth thing")?
                .parse::<i32>()
                .map_err(|_| "oh no")?;
            return Ok(Command::Addx(n));
        } else {
            return Err("Invalid command. Not noop or addx".to_string());
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let commands = fs::read_to_string("src/d10/input")?
        .lines()
        .map(Command::try_from)
        .collect::<Result<Vec<Command>, String>>()?;

    let mut state = State {
        register_x: 0,
        cycle_num: 0,
    };

    let cycle_num_checkpoints = vec![20, 60, 100, 140, 180, 220];
    let mut signal_strength = 0;
    for command in commands.iter() {
        let prev_cycle_num = state.cycle_num;
        match command {
            Command::Noop => state.cycle_num += 1,
            Command::Addx(n) => {
                state.cycle_num += 2;
                state.register_x += n;
            }
        }

        for &checkpoint in cycle_num_checkpoints.iter() {
            if state.cycle_num >= checkpoint && prev_cycle_num < checkpoint {
                println!(
                    "prev_cycle={} current_cycle={} checkpoint={} register_x={}",
                    prev_cycle_num, state.cycle_num, checkpoint, state.register_x
                );
                signal_strength += checkpoint * state.register_x;
            }
        }
    }
    println!("{}", signal_strength);

    return Ok(());
}
