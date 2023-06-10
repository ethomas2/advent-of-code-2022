use std::error::Error;
use std::fs;
use std::ops::Add;

#[derive(Copy, Clone)]
struct Loc(i64, i64);

impl Add for Loc {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        let Loc(r1, c1) = self;
        let Loc(r2, c2) = other;
        return Loc(r1 + r2, c1 + c2);
    }
}

impl Add<Direction> for Loc {
    type Output = Self;
    fn add(self, other: Direction) -> Loc {
        let x = match other {
            Direction::North => Loc(-1, 0),
            Direction::South => Loc(1, 0),
            Direction::East => Loc(0, 1),
            Direction::West => Loc(0, -1),
        };
        self + x
    }
}

#[derive(Copy, Clone)]
enum Item {
    Empty,
    Elf,
}

enum Direction {
    North,
    South,
    East,
    West,
}

impl Item {
    fn from_char(ch: char) -> Result<Self, String> {
        match ch {
            '.' => Ok(Item::Empty),
            '#' => Ok(Item::Elf),
            _ => Err(format!("Can't convert char \"{}\" to Item", ch)),
        }
    }

    fn to_char(&self) -> char {
        match &self {
            &Item::Empty => '.',
            &Item::Elf => '#',
        }
    }
}

struct Grid {
    grid: Vec<Item>,
    width: usize,
    height: usize,
}

impl<'a> TryFrom<&'a str> for Grid {
    type Error = String;

    fn try_from(content: &'a str) -> Result<Self, Self::Error> {
        let height = content.lines().count();
        let width = content.lines().next().ok_or(String::from("oh no"))?.len();
        let grid = content
            .lines()
            .flat_map(|line| line.chars().map(Item::from_char))
            .collect::<Result<Vec<Item>, String>>()?;
        Ok(Grid {
            grid,
            height,
            width,
        })
    }
}

impl Grid {
    fn get(&self, loc: Loc) -> Option<Item> {
        let Loc(row, col) = loc;
        if row < 0 || col < 0 {
            return None;
        }
        let idx = (row as usize) * self.width + (col as usize);
        if idx <= self.grid.len() {
            Some(self.grid[idx])
        } else {
            None
        }
    }

    fn pfmt(&self) -> String {
        let mut s = String::from("");
        for r in 0..self.width {
            for c in 0..self.height {
                s += self
                    .get(Loc(r as i64, c as i64))
                    .unwrap()
                    .to_char()
                    .to_string()
                    .as_str();
            }
            s += "\n"
        }
        s
    }

    fn empty(width: usize, height: usize) -> Self {
        Grid {
            width,
            height,
            grid: vec![Item::Empty; width * height],
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string("src/d23/input")?;
    let grid = Grid::try_from(content.as_str())?;
    println!("{}", grid.pfmt());

    let mut propose_order = vec![
        Direction::North,
        Direction::South,
        Direction::West,
        Direction::East,
    ];

    for i in 0..10 {
        // propose
        // move
        // print
    }

    Ok(())
}
