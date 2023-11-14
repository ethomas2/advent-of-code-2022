use std::convert::TryFrom;
use std::error::Error;
use std::fs;

#[derive(Clone)]
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
        register_x: 1,
        cycle_num: 1,
    };

    let state_iter = commands.iter().flat_map(|command| match command {
        &Command::Noop => {
            state.cycle_num += 1;
            let x: Box<dyn Iterator<Item = State>> =
                Box::new(std::iter::empty().chain(std::iter::once(state.clone())));
            return x;
        }
        &Command::Addx(n) => {
            state.cycle_num += 1;
            let state1 = state.clone();

            state.cycle_num += 1;
            state.register_x += n;
            let state2 = state.clone();

            let x: Box<dyn Iterator<Item = State>> =
                Box::new(std::iter::once(state1).chain(std::iter::once(state2)));
            return x;
        }
    });

    let signal_stength: i32 = state_iter
        .filter(|State { cycle_num, .. }| matches!(cycle_num, 20 | 60 | 100 | 140 | 180 | 220))
        .map(
            |State {
                 cycle_num,
                 register_x,
             }| cycle_num * register_x,
        )
        .sum();

    println!("{}", signal_stength);

    return Ok(());
}
