use std::fs;

struct Range {
    low: usize,
    high: usize,
}

impl Range {
    fn from_str(s: &str) -> Result<Self, String> {
        let v = s.split('-').collect::<Vec<&str>>();
        if v.len() != 2 {
            return Err(String::from("Expected exactly one '-'"));
        }

        let low = v[0].parse::<usize>().map_err(|err| err.to_string())?;
        let high = v[1].parse::<usize>().map_err(|err| err.to_string())?;

        return Ok(Range { low, high });
    }

    fn contains(&self, other: &Self) -> bool {
        return self.low <= other.low && self.high >= other.high;
    }

    fn overlap(&self, other: &Self) -> bool {
        return !(self.high < other.low || self.low > other.high);
    }
}

fn part1() -> std::io::Result<()> {
    let content = fs::read_to_string("src/d04/input")?;
    let lines = content.lines();
    let num = lines
        .filter(|line| {
            let v = line.split(',').collect::<Vec<&str>>();
            assert!(v.len() == 2);
            let (left, right) = (
                Range::from_str(v[0]).unwrap(),
                Range::from_str(v[1]).unwrap(),
            );
            let contains = left.contains(&right) || right.contains(&left);
            println!("{} {}", line, contains);
            contains
        })
        .count();
    println!("{}", num);
    Ok(())
}

fn part2() -> std::io::Result<()> {
    let content = fs::read_to_string("src/d04/input")?;
    let lines = content.lines();
    let num = lines
        .filter(|line| {
            let v = line.split(',').collect::<Vec<&str>>();
            assert!(v.len() == 2);
            let (left, right) = (
                Range::from_str(v[0]).unwrap(),
                Range::from_str(v[1]).unwrap(),
            );
            let overlap = left.overlap(&right);
            println!("{} {}", line, overlap);
            overlap
        })
        .count();
    println!("{}", num);
    Ok(())
}

fn main() -> std::io::Result<()> {
    Ok(part2()?)
}
