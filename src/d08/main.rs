use std::error::Error;
use std::fs;

struct Grid(Vec<Vec<u8>>);

impl<'a> TryFrom<&'a str> for Grid {
    type Error = String;
    fn try_from(input: &'a str) -> Result<Grid, Self::Error> {
        let x: Vec<Vec<u8>> = input
            .lines()
            .map(|line| {
                let x = line
                    .chars()
                    .map(|c| {
                        let x = c.to_string().parse::<u8>();
                        return x;
                    })
                    .collect::<Result<Vec<u8>, std::num::ParseIntError>>();
                return x;
            })
            .collect::<Result<Vec<Vec<u8>>, std::num::ParseIntError>>()
            .map_err(|err| err.to_string())?;

        Ok(Grid(x))
    }
}

fn part1() -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string("src/d08/input")?;
    let grid = Grid::try_from(content.as_str())?.0; // TODO: why do i need this?
    let nrows = grid.len();
    let ncols = grid[0].len();
    let mut visible: Vec<Vec<bool>> = vec![vec![false; ncols]; nrows];

    for r in 0..nrows {
        visible[r][0] = true;
        let mut b = grid[r][0];
        for c in 1..ncols {
            if grid[r][c] > b {
                visible[r][c] = true;
            }
            b = std::cmp::max(grid[r][c], b);
        }
    }

    for r in 0..nrows {
        visible[r][ncols - 1] = true;
        let mut b = grid[r][ncols - 1];
        for c in (0..=(ncols - 2)).rev() {
            if grid[r][c] > b {
                visible[r][c] = true;
            }
            b = std::cmp::max(grid[r][c], b)
        }
    }

    for c in 0..ncols {
        visible[0][c] = true;
        let mut b = grid[0][c];
        for r in 1..nrows {
            if grid[r][c] > b {
                visible[r][c] = true;
            }
            b = std::cmp::max(grid[r][c], b);
        }
    }

    for c in 0..ncols {
        visible[nrows - 1][c] = true;
        let mut b = grid[nrows - 1][c];
        for r in (0..=(nrows - 2)).rev() {
            if grid[r][c] > b {
                visible[r][c] = true;
            }
            b = std::cmp::max(grid[r][c], b);
        }
    }

    let num_visible = visible
        .iter()
        .map(|row| row.iter())
        .flatten()
        .filter(|x| **x)
        .count();
    println!("{}", num_visible);

    Ok(())
}

fn part2() -> Result<(), Box<dyn Error>> {
    let content = std::fs::read_to_string("src/d08/input")?;
    let grid = Grid::try_from(content.as_str())?.0;

    let mut best = 0;
    for r in 0..(grid.len() as isize) {
        for c in 0..(grid[r as usize].len() as isize) {
            let mut lr: isize = r;
            let mut rr: isize = r;
            let mut lc: isize = c;
            let mut rc: isize = c;
            while lr > 0 && grid[lr as usize][c as usize] < grid[r as usize][c as usize] {
                lr -= 1
            }
            while rr < grid.len() as isize
                && grid[rr as usize][c as usize] < grid[r as usize][c as usize]
            {
                rr += 1
            }
            while lc >= 0 && grid[r as usize][lc as usize] < grid[r as usize][c as usize] {
                lc -= 1
            }
            while rc < grid.len() as isize
                && grid[r as usize][rc as usize] < grid[r as usize][c as usize]
            {
                rc += 1
            }

            let this = (r - lr) * (rr - r) * (c - lc) * (rc - c);
            if this > best {
                println!("{} {} {}", r, c, this);
                println!("{} {} {}", r, c, this);
                best = this;
            }
        }
    }
    println!("{}", best);

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    part2()
}
