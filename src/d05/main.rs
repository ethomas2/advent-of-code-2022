use itertools::Itertools;
use regex::Regex;
use std::error::Error;
use std::fs;
use std::str::Lines;

type Stack = Vec<char>;

#[derive(Debug)]
struct BoardState {
    stacks: Vec<Stack>,
}
impl BoardState {
    // TODO: why do i need the lifetime parameter here? how could the &str have a lifetime less
    // than from_lines?
    fn from_lines<'a>(lines: impl Iterator<Item = &'a str>) -> Self {
        let grid: Vec<Vec<char>> = lines
            .map(|line| line.chars().map(|x| x).collect::<Stack>())
            .collect();
        let nrows = grid.len();
        let last_line = grid.last().unwrap();
        let col_indicies =
            last_line
                .iter()
                .enumerate()
                .filter_map(|(idx, ch)| if *ch == ' ' { None } else { Some(idx) });
        let stacks = col_indicies
            .map(|col_idx| {
                let row_indicies = (0..=(nrows - 2)).rev().take_while(|&row_idx| {
                    col_idx <= grid[row_idx].len() && grid[row_idx][col_idx] != ' '
                });
                let stack = row_indicies
                    .map(|row_idx| grid[row_idx][col_idx])
                    .collect_vec();
                stack
            })
            .collect_vec();
        return BoardState { stacks };
    }
}

#[derive(Debug, Copy, Clone)]
struct Command {
    num: usize,
    from: usize,
    to: usize,
}
impl Command {
    fn from_str(s: &str) -> Result<Command, String> {
        let re = Regex::new("move (\\d+) from (\\d+) to (\\d+)").map_err(|err| err.to_string())?;
        let caps = re.captures(s).ok_or(String::from("Invalid Strign"))?;

        let num = caps[1].parse::<usize>().map_err(|e| e.to_string())?;
        let from = caps[2].parse::<usize>().map_err(|e| e.to_string())? - 1;
        let to = caps[3].parse::<usize>().map_err(|e| e.to_string())? - 1;

        Ok(Command { num, from, to })
    }
}

fn execute_command_flip_order(mut boardstate: BoardState, command: Command) -> BoardState {
    let Command { num, from, to } = command;
    for _ in 1..=num {
        // TODO: don't unwrap
        let x = boardstate.stacks[from].pop().unwrap();
        boardstate.stacks[to].push(x);
    }

    return boardstate;
}

fn execute_command_maintain_order(mut boardstate: BoardState, command: Command) -> BoardState {
    let Command { num, from, to } = command;
    let from_stack_len = boardstate.stacks[from].len();
    let chunk = boardstate.stacks[from]
        .splice((from_stack_len - num)..from_stack_len, vec![])
        .collect_vec();
    boardstate.stacks[to].extend(chunk);

    return boardstate;
}

fn part1() -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string("src/d05/input")?;
    let mut lines = content.lines();
    let first_lines = lines.by_ref().take_while(|&line| line != "");
    let boardstate = BoardState::from_lines(first_lines);
    let commands = lines
        .map(Command::from_str)
        .collect::<Result<Vec<Command>, String>>()?;
    let new_boardstate = commands
        .into_iter()
        .fold(boardstate, execute_command_flip_order);
    let final_chars = String::from_iter(
        new_boardstate
            .stacks
            .iter()
            .map(|stack| stack.last().unwrap()),
    );
    println!("{}", final_chars);
    Ok(())
}

fn part2() -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string("src/d05/input")?;
    let mut lines = content.lines();
    let first_lines = lines.by_ref().take_while(|&line| line != "");
    let boardstate = BoardState::from_lines(first_lines);
    let commands = lines
        .map(Command::from_str)
        .collect::<Result<Vec<Command>, String>>()?;
    let new_boardstate = commands
        .into_iter()
        .fold(boardstate, execute_command_maintain_order);
    let final_chars = String::from_iter(
        new_boardstate
            .stacks
            .iter()
            .map(|stack| stack.last().unwrap()),
    );
    println!("{}", final_chars);
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    return part2();
}
