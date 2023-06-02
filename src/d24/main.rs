use std::collections::HashMap;
use std::collections::HashSet;
use std::error::Error;
use std::fs;
use std::iter::FromIterator;

#[derive(PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
#[derive(PartialEq, Eq)]
enum Space {
    Empty,
    Blizzard(Direction),
    Wall,
}

type I = i64;
fn modulus(a: I, b: I) -> I {
    return ((a % b) + b) % b;
}

fn main() -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string("src/d24/input")?;
    // parse string to a hashset of tuples
    let startgrid: HashMap<(I, I), Space> = content
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars().enumerate().map(move |(col, ch)| match ch {
                '>' => ((row as I, col as I), Space::Blizzard(Direction::Right)),
                '<' => ((row as I, col as I), Space::Blizzard(Direction::Left)),
                'v' => ((row as I, col as I), Space::Blizzard(Direction::Down)),
                '^' => ((row as I, col as I), Space::Blizzard(Direction::Up)),
                '#' => ((row as I, col as I), Space::Wall),
                '.' => ((row as I, col as I), Space::Empty),
                _ => panic!("unexpected char {}", ch), // TODO: make error
            })
        })
        .collect::<_>();
    let startloc: (I, I) = *startgrid
        .iter()
        .find(|&(&(row, _), space)| row == 0 && *space == Space::Empty)
        .unwrap()
        .0;
    let height = content.lines().count() as I;
    let width = content.lines().next().unwrap().len() as I;
    let endloc: (I, I) = *startgrid
        .iter()
        .find(|&(&(_, col), space)| col == (height - 1) && *space == Space::Empty)
        .unwrap()
        .0;

    // spread over and remove
    // t=0 is the first frame
    let mut possible_locations: HashSet<(I, I)> = HashSet::from_iter(vec![startloc]);
    for t in 1.. {
        let next_possible_locations: HashSet<(I, I)> = possible_locations
            .iter()
            .flat_map(|&(row, col)| {
                dbg!((t, row, col));
                let newlocs = if row == 0 {
                    vec![(row, col), (row + 1, col)]
                } else {
                    vec![
                        (row, col),
                        (row + 1, col),
                        (row - 1, col),
                        (row, col + 1),
                        (row, col - 1),
                    ]
                };

                newlocs
                    .into_iter()
                    .map(|(r, c)| {
                        if (r, c) == startloc || (r, c) == endloc {
                            return (r, c);
                        }
                        (
                            modulus(r - 1, height - 2) + 1,
                            modulus(c - 1, width - 2) + 1,
                        )
                    })
                    .filter(|&(r, c)| {
                        if (r, c) == startloc || (r, c) == endloc {
                            return true;
                        }
                        let uploc = (modulus((r - 1 - t), (height - 2)) + 1, c);
                        let downloc = (modulus(r - 1 + t, height - 2) + 1, c);
                        let leftloc = (r, modulus((c - 1 - t), (width - 2)) + 1);
                        let rightloc = (r, modulus((c - 1 + t), (width - 2)) + 1);
                        let works = (*startgrid.get(&uploc).unwrap()
                            != Space::Blizzard(Direction::Down))
                            && (*startgrid.get(&downloc).unwrap()
                                != Space::Blizzard(Direction::Up))
                            && (*startgrid.get(&leftloc).unwrap()
                                != Space::Blizzard(Direction::Right))
                            && (*startgrid.get(&rightloc).unwrap()
                                != Space::Blizzard(Direction::Left));
                        works
                    })
            })
            .collect::<_>();
        if next_possible_locations.contains(&endloc) {
            println!("{}", t);
            break;
        }
        possible_locations = next_possible_locations;
    }
    Ok(())
}
