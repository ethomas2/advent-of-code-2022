use itertools::Itertools;
use std::fs::File;
use std::io::prelude::*;

fn part1() -> std::io::Result<()> {
    let mut file = File::open("input")?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let vec = contents
        .lines()
        .group_by(|&s| s == "")
        .into_iter()
        .filter_map(|(is_empty, s)| {
            if is_empty {
                None
            } else {
                Some(s.collect::<Vec<&str>>())
            }
        })
        .map(|strs| strs.iter().map(|s| s.parse::<i32>().unwrap()).sum::<i32>())
        .max()
        .unwrap();

    println!("{:#?}", vec);
    Ok(())
}

fn part2() -> std::io::Result<()> {
    let mut file = File::open("input")?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let vec: i32 = contents
        .lines()
        .group_by(|&s| s == "")
        .into_iter()
        .filter_map(|(is_empty, s)| {
            if is_empty {
                None
            } else {
                Some(s.collect::<Vec<&str>>())
            }
        })
        .map(|strs| strs.iter().map(|s| s.parse::<i32>().unwrap()).sum::<i32>())
        .sorted_by_key(|n| -n)
        .take(3)
        .sum();

    println!("{:#?}", vec);
    Ok(())
}

fn main() -> std::io::Result<()> {
    return part2();
}
