use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn priority(ch: char) -> Option<i32> {
    let chars = ('a'..='z').chain('A'..='Z');
    let priorities = 1..=52;
    let map: HashMap<char, i32> = chars.zip(priorities).collect();
    return map.get(&ch).copied();
}

fn part1() -> std::io::Result<()> {
    let file = File::open("src/d03/input")?;
    let reader = BufReader::new(file);
    let result: i32 = reader
        .lines()
        .collect::<Result<Vec<String>, _>>()?
        .into_iter()
        .map(|line| {
            assert!(line.len() % 2 == 0, "Line length is not even");
            let (head, tail) = line.split_at(line.len() / 2);
            // TODO: could i use HashSet::from here?
            let head_set: HashSet<char> = head.chars().collect();
            let tail_set: HashSet<char> = tail.chars().collect();
            let intersection: Vec<char> = head_set
                .intersection(&tail_set)
                .into_iter()
                .map(|x| *x)
                .collect();
            assert!(intersection.len() == 1);
            println!(
                "intersection:: {}, {}",
                intersection[0],
                priority(intersection[0]).unwrap()
            );
            return priority(intersection[0]).unwrap();
        })
        .sum();
    println!("{:?}", result);
    Ok(())
}

fn main() -> std::io::Result<()> {
    part1()
}
