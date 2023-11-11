use std::collections::HashSet;
use std::convert::TryFrom;
use std::error::Error;
use std::fs;

struct Rope {
    head: (isize, isize),
    tail: (isize, isize),
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    Up,
    Left,
    Right,
    Down,
}

impl TryFrom<&str> for Direction {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        return match value {
            "U" => Ok(Direction::Up),
            "L" => Ok(Direction::Left),
            "R" => Ok(Direction::Right),
            "D" => Ok(Direction::Down),
            _ => Err(String::from("oh no")),
        };
    }
}

fn move_rope(mut rope: Rope, dir: Direction) -> Rope {
    use Direction::*;
    let Rope {
        head: (ref mut head_r, ref mut head_c),
        tail: (ref mut tail_r, ref mut tail_c),
    } = rope;
    match dir {
        Up => {
            *head_r -= 1;
        }
        Down => {
            *head_r += 1;
        }
        Left => {
            *head_c -= 1;
        }
        Right => {
            *head_c += 1;
        }
    }
    if (*head_r - *tail_r).abs() > 1 || (*head_c - *tail_c).abs() > 1 {
        let rdir = (*head_r - *tail_r).signum();
        let cdir = (*head_c - *tail_c).signum();
        *tail_r += rdir;
        *tail_c += cdir;
    }

    return rope;
}

fn print_rope(rope: &Rope, width: isize, height: isize) {
    let Rope {
        head: (head_r, head_c),
        tail: (tail_r, tail_c),
    } = rope;
    for r in 0..height {
        for c in 0..width {
            if (&r, &c) == (head_r, head_c) {
                print!("H");
            } else if (&r, &c) == (tail_r, tail_c) {
                print!("T");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let parsed_input = fs::read_to_string("src/d09/input")?
        .lines()
        .map(|line| {
            let s = line.split(' ').collect::<Vec<&str>>();
            let dir = Direction::try_from(s[0])?;
            let n = s[1].parse::<isize>().map_err(|_| String::from("oh no"))?;
            return Ok((dir, n));
        })
        .collect::<Result<Vec<(Direction, isize)>, String>>()?;
    let start = (5, 5);
    let mut rope = Rope {
        head: start,
        tail: start,
    };
    print_rope(&rope, 10, 10);

    let mut unique_positions: HashSet<(isize, isize)> = HashSet::new();
    unique_positions.insert(rope.tail);
    for (dir, n) in parsed_input.iter() {
        for _ in 0..*n {
            println!("{:?}", *dir);
            rope = move_rope(rope, *dir);
            unique_positions.insert(rope.tail);
        }
    }
    println!("{}", unique_positions.len());
    return Ok(());
}
