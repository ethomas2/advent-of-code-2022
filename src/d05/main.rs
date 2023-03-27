use itertools::Itertools;
use std::fs;
use std::str::Lines;

type Stack = Vec<char>;

#[derive(Debug)]
struct Diagram {
    stacks: Vec<Stack>,
}
impl Diagram {
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
                // let mut stack: Vec<char> =  Vec::new();
                let row_indicies = (0..=(nrows - 2)).rev().take_while(|&row_idx| {
                    col_idx <= grid[row_idx].len() && grid[row_idx][col_idx] != ' '
                });
                let stack = row_indicies
                    .map(|row_idx| grid[row_idx][col_idx])
                    .collect_vec();
                stack
            })
            .collect_vec();
        return Diagram { stacks };
    }
}

#[derive(Debug)]
struct Command {}
impl Command {
    fn from_str(s: &str) -> Result<Command, String> {
        println!("from_str, {}", s);
        Ok(Command {})
    }
}

fn main() -> std::io::Result<()> {
    // split into diagram and commands
    // diagram
    //    parse into 2d array of chars
    //    bottom row numbers is index of columns
    //    for each column index, initialize a stack and push elements on it from bottom to top

    let content = fs::read_to_string("src/d05/input")?;
    let mut lines = content.lines();
    let first_lines = lines.by_ref().take_while(|&line| line != "");
    let diagram = Diagram::from_lines(first_lines);
    println!("{:?}", diagram);
    let commands = lines.map(Command::from_str).collect::<Vec<_>>();
    // println!("{:?}", commands);
    Ok(())
}
