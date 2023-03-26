use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
// use std::io::prelude::*;

#[derive(Clone, Copy)]
enum Outcome {
    Win = 0,
    Loss = 1,
    Draw = 2,
}
impl Outcome {
    fn from_str(ch: &str) -> Option<Self> {
        let map: HashMap<&str, Self> = vec![("X", Self::Loss), ("Y", Self::Draw), ("Z", Self::Win)]
            .into_iter()
            .collect::<HashMap<&str, Self>>();
        return map.get(ch).map(|x| *x);
    }

    fn score(&self) -> i32 {
        match self {
            Self::Win => 6,
            Self::Draw => 3,
            Self::Loss => 0,
        }
    }
}

// TODO: in fasterthanlime's version does he derive PartialEq and Eq?
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum RPS {
    Rock,
    Paper,
    Scissors,
}

impl RPS {
    pub fn from_str(s: &str) -> Option<Self> {
        // let map: HashMap<&str, Self> = vec![
        //     ("A", Self::Rock),
        //     ("B", Self::Paper),
        //     ("C", Self::Scissors),
        //     ("X", Self::Rock),
        //     ("Y", Self::Paper),
        //     ("Z", Self::Scissors),
        // ]
        // .into_iter()
        // .collect();
        // return map.get(s).map(|x| *x);

        match s {
            "A" => Some(Self::Rock),
            "B" => Some(Self::Paper),
            "C" => Some(Self::Scissors),
            "X" => Some(Self::Rock),
            "Y" => Some(Self::Paper),
            "Z" => Some(Self::Scissors),
            _ => None,
        }
    }

    pub fn score(&self) -> i32 {
        // let map: HashMap<Self, i32> = vec![(Self::Rock, 1), (Self::Paper, 2), (Self::Scissors, 3)]
        //     .into_iter()
        //     .collect();
        // return *map.get(self).unwrap();

        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }

    pub fn beats(&self, other: RPS) -> bool {
        match (self, other) {
            (RPS::Rock, RPS::Scissors) => true,
            (RPS::Scissors, RPS::Paper) => true,
            (RPS::Paper, RPS::Rock) => true,
            _ => false,
        }
    }

    pub fn outcome_score(&self, other: RPS) -> i32 {
        if *self == other {
            return 3;
        } else if self.beats(other) {
            return 6;
        } else {
            return 0;
        }
    }
}

fn part1() -> std::io::Result<i32> {
    let mut file = File::open("src/d02/input")?;
    let buf = BufReader::new(file);
    let total: i32 = buf
        .lines()
        .map(|line| line)
        // TODO: do i have to collect into a vec here?
        .collect::<std::io::Result<Vec<String>>>()?
        .iter()
        .map(|line| {
            // let [mine, theirs] = line.split(' ').collect::<Vec<&str>>();
            let foo = line.split(' ').collect::<Vec<&str>>();
            let theirs = RPS::from_str(foo[0]).unwrap();
            let mine = RPS::from_str(foo[1]).unwrap();
            // TODO: can i pattern match this?
            let score = mine.score() + mine.outcome_score(theirs);
            score
        })
        .sum();
    Ok(total)
}

fn part2() -> std::io::Result<i32> {
    let mut file = File::open("src/d02/input")?;
    let reader = BufReader::new(file);
    let total = reader
        .lines()
        .collect::<std::io::Result<Vec<_>>>()?
        .iter()
        .map(|line| {
            use Outcome::*;
            use RPS::*;
            let foo: Vec<_> = line.split(' ').collect();
            let opponent = RPS::from_str(foo[0]).unwrap();
            let outcome = Outcome::from_str(foo[1]).unwrap();
            let mine = match (opponent, outcome) {
                (Rock, Draw) | (Scissors, Win) | (Paper, Loss) => Rock,
                (Paper, Draw) | (Rock, Win) | (Scissors, Loss) => Paper,
                (Scissors, Draw) | (Paper, Win) | (Rock, Loss) => Scissors,
            };
            return mine.score() + outcome.score();
        })
        .sum::<i32>();

    Ok(total)
}

fn main() -> std::io::Result<()> {
    // println!("{}", part1()?);
    println!("{}", part2()?);
    // TODO: how to make the following less verbose
    // let unknown_error = std::io::Error::new(std::io::ErrorKind::Other, "oh no");
    // let x = RPS::from_str("X").ok_or(unknown_error)?;
    // println!("{:?}", x);
    Ok(())
}
