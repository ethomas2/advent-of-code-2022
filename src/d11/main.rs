use either::Either;
use itertools::process_results;
use regex::Regex;
use std::collections::HashMap;
use std::error::Error;
use std::fs;

#[derive(Debug, Clone, Copy)]
enum Op {
    Add,
    Sub,
    Mul,
}

#[derive(Debug, Clone, Copy)]
struct Old;

#[derive(Debug)]
struct Operation {
    lhs: Either<Old, i64>,
    rhs: Either<Old, i64>,
    op: Op,
}

impl Operation {
    fn call(&self, old_item: i64) -> i64 {
        let [lhs, rhs]: [i64; 2] = [self.lhs, self.rhs].map(|x| match x {
            Either::Left(Old) => old_item,
            Either::Right(y) => y,
        });

        match self.op {
            Op::Add => lhs + rhs,
            Op::Sub => lhs - rhs,
            Op::Mul => dbg!(lhs) * dbg!(rhs),
        }
    }
}

#[derive(Debug)]
struct Monkey {
    id: i64,
    items: Vec<i64>,
    operation: Operation,
    divisible: i64,
    if_true: i64,
    if_false: i64,
}

impl Monkey {
    fn test(&self, item: i64) -> i64 {
        if item % self.divisible == 0 {
            return self.if_true;
        } else {
            return self.if_false;
        }
    }
}

fn parse_monkey(s: &str) -> Result<Monkey, Box<dyn Error>> {
    let pattern = r#"Monkey (?P<id>\d+):
  Starting items: (?P<items>.*)
  Operation: new = (?P<operation>.*)
  Test: divisible by (?P<divisible>\d+)
    If true: throw to monkey (?P<if_true>\d+)
    If false: throw to monkey (?P<if_false>\d+)"#;

    let cap = Regex::new(pattern)?.captures(s).ok_or("bad capture")?;
    let (id, items, operation, divisible, if_true, if_false) = (
        cap.name("id").ok_or("bad id")?.as_str().parse::<i64>()?,
        cap.name("items").ok_or(":o")?.as_str(),
        cap.name("operation").ok_or(":o")?.as_str(),
        cap.name("divisible").ok_or(":o")?.as_str().parse::<i64>()?,
        cap.name("if_true").ok_or(":o")?.as_str().parse::<i64>()?,
        cap.name("if_false").ok_or(":o")?.as_str().parse::<i64>()?,
    );

    let items = items
        .split(' ')
        .map(|s| s.replace(',', "").parse::<i64>().unwrap()) // TODO: get rid of unwrap
        .collect::<Vec<i64>>();

    let isnum = |x: &str| -> bool { x.chars().all(|c| c.is_numeric()) };
    let operation: Operation = {
        let pieces = operation.split(" ").collect::<Vec<&str>>();
        let op = match pieces[1] {
            "+" => Op::Add,
            "-" => Op::Sub,
            "*" => Op::Mul,
            _ => panic!("unexpected piece {}", pieces[1]),
        };
        let [lhs, rhs]: [Either<Old, i64>; 2] = [pieces[0], pieces[2]].map(|x| match x {
            "old" => Either::Left(Old),
            _ if isnum(x) => Either::Right(x.parse::<i64>().expect(":o")),
            _ => panic!(":o"),
        });
        Operation { lhs, rhs, op }
    };

    let monkey = Monkey {
        id,
        items,
        operation,
        divisible,
        if_true,
        if_false,
    };
    Ok(monkey)
}

fn parse(content: &str) -> Result<MonkeyMap, Box<dyn Error>> {
    let parsed_monkeys = content.split("\n\n").map(parse_monkey);
    let map = process_results(parsed_monkeys, |iter| {
        let map: MonkeyMap = iter.map(|monkey| (monkey.id, monkey)).collect();
        map
    })?;

    Ok(map)
}

type MonkeyMap = HashMap<i64, Monkey>;

fn take_turn<F>(map: &mut MonkeyMap, id: i64, mut closure: F)
where
    F: FnMut(&MonkeyMap, i64),
{
    // for each item (<item_i>)
    //   - operation(<item_i>)
    //   - <item_i> /= 3
    //   - test worry level <item_i> and throw

    while map.get_mut(&id).unwrap().items.len() > 0 {
        // mutate the src monkey
        let src_monkey = map.get_mut(&id).unwrap();
        src_monkey.items[0] = src_monkey.operation.call(src_monkey.items[0]);
        src_monkey.items[0] /= 3;
        let throw_to = src_monkey.test(src_monkey.items[0]); // why does this work? taking an
                                                             // immutable reference out while
                                                             // already have a mutable one
        let item_to_throw = src_monkey.items.remove(0);

        // mutate the dst monkey
        let dst_monkey = map.get_mut(&throw_to).unwrap();
        dst_monkey.items.push(item_to_throw);
        closure(map, id);
    }
}

fn take_round<F>(map: &mut MonkeyMap, mut closure: F)
where
    F: FnMut(&MonkeyMap, i64),
{
    for id in 0..map.len() {
        take_turn(map, id as i64, &mut closure);
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string("src/d11/input")?;
    let mut monkey_map = parse(content.as_str())?;
    let mut inspection_log: HashMap<i64, i64> = HashMap::new(); // monkey id -> num times inspected
    let mut closure = |mm: &MonkeyMap, id: i64| {
        *inspection_log.entry(id).or_insert(0) += 1;
    };
    for _ in 0..20 {
        take_round(&mut monkey_map, &mut closure);
    }
    let mut inspections: Vec<i64> = inspection_log.iter().map(|(_, val)| *val).collect::<_>();
    inspections.sort();
    inspections.reverse();
    match inspections[0..2] {
        [a, b] => {
            println!("{}", a);
            println!("{}", b);
            println!("{}", a * b);
        }
        _ => println!("Bad inspections"),
    }
    // println!("{:?}", inspection_log);
    // println!("{}", monkey_buisness);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn _get_data() -> MonkeyMap {
        HashMap::from([
            (
                0,
                Monkey {
                    id: 0,
                    items: vec![79, 98],
                    operation: Operation {
                        lhs: Either::Left(Old),
                        rhs: Either::Right(19),
                        op: Op::Mul,
                    },
                    divisible: 23,
                    if_true: 2,
                    if_false: 3,
                },
            ),
            (
                1,
                Monkey {
                    id: 1,
                    items: vec![54, 65, 75, 74],
                    operation: Operation {
                        lhs: Either::Left(Old),
                        rhs: Either::Right(6),
                        op: Op::Add,
                    },
                    divisible: 19,
                    if_true: 2,
                    if_false: 0,
                },
            ),
            (
                2,
                Monkey {
                    id: 2,
                    items: vec![79, 60, 97],
                    operation: Operation {
                        lhs: Either::Left(Old),
                        rhs: Either::Left(Old), // 'old * old' implies both lhs and rhs are the old value
                        op: Op::Mul,
                    },
                    divisible: 13,
                    if_true: 1,
                    if_false: 3,
                },
            ),
            (
                3,
                Monkey {
                    id: 3,
                    items: vec![74],
                    operation: Operation {
                        lhs: Either::Left(Old),
                        rhs: Either::Right(3),
                        op: Op::Add,
                    },
                    divisible: 17,
                    if_true: 0,
                    if_false: 1,
                },
            ),
        ])
    }

    #[test]
    fn test_1() {
        let mut monkey_map = _get_data();

        for round in 1..=10 {
            take_round(&mut monkey_map, |_| {});

            match round {
                2 => {
                    assert_eq!(
                        monkey_map.get(&0).unwrap().items,
                        vec![695, 10, 71, 135, 350]
                    );
                    assert_eq!(monkey_map.get(&1).unwrap().items, vec![43, 49, 58, 55, 362]);
                    assert_eq!(monkey_map.get(&2).unwrap().items, vec![]);
                    assert_eq!(monkey_map.get(&3).unwrap().items, vec![]);
                }
                3 => {
                    assert_eq!(monkey_map.get(&0).unwrap().items, vec![16, 18, 21, 20, 122]);
                    assert_eq!(
                        monkey_map.get(&1).unwrap().items,
                        vec![1468, 22, 150, 286, 739]
                    );
                    assert_eq!(monkey_map.get(&2).unwrap().items, vec![]);
                    assert_eq!(monkey_map.get(&3).unwrap().items, vec![]);
                }
                4 => {
                    assert_eq!(
                        monkey_map.get(&0).unwrap().items,
                        vec![491, 9, 52, 97, 248, 34]
                    );
                    assert_eq!(monkey_map.get(&1).unwrap().items, vec![39, 45, 43, 258]);
                    assert_eq!(monkey_map.get(&2).unwrap().items, vec![]);
                    assert_eq!(monkey_map.get(&3).unwrap().items, vec![]);
                }
                5 => {
                    assert_eq!(
                        monkey_map.get(&0).unwrap().items,
                        vec![15, 17, 16, 88, 1037]
                    );
                    assert_eq!(
                        monkey_map.get(&1).unwrap().items,
                        vec![20, 110, 205, 524, 72]
                    );
                    assert_eq!(monkey_map.get(&2).unwrap().items, vec![]);
                    assert_eq!(monkey_map.get(&3).unwrap().items, vec![]);
                }
                6 => {
                    assert_eq!(monkey_map.get(&0).unwrap().items, vec![8, 70, 176, 26, 34]);
                    assert_eq!(
                        monkey_map.get(&1).unwrap().items,
                        vec![481, 32, 36, 186, 2190]
                    );
                    assert_eq!(monkey_map.get(&2).unwrap().items, vec![]);
                    assert_eq!(monkey_map.get(&3).unwrap().items, vec![]);
                }
                7 => {
                    assert_eq!(
                        monkey_map.get(&0).unwrap().items,
                        vec![162, 12, 14, 64, 732, 17]
                    );
                    assert_eq!(monkey_map.get(&1).unwrap().items, vec![148, 372, 55, 72]);
                    assert_eq!(monkey_map.get(&2).unwrap().items, vec![]);
                    assert_eq!(monkey_map.get(&3).unwrap().items, vec![]);
                }
                8 => {
                    assert_eq!(
                        monkey_map.get(&0).unwrap().items,
                        vec![51, 126, 20, 26, 136]
                    );
                    assert_eq!(
                        monkey_map.get(&1).unwrap().items,
                        vec![343, 26, 30, 1546, 36]
                    );
                    assert_eq!(monkey_map.get(&2).unwrap().items, vec![]);
                    assert_eq!(monkey_map.get(&3).unwrap().items, vec![]);
                }
                9 => {
                    assert_eq!(
                        monkey_map.get(&0).unwrap().items,
                        vec![116, 10, 12, 517, 14]
                    );
                    assert_eq!(
                        monkey_map.get(&1).unwrap().items,
                        vec![108, 267, 43, 55, 288]
                    );
                    assert_eq!(monkey_map.get(&2).unwrap().items, vec![]);
                    assert_eq!(monkey_map.get(&3).unwrap().items, vec![]);
                }
                10 => {
                    assert_eq!(monkey_map.get(&0).unwrap().items, vec![91, 16, 20, 98]);
                    assert_eq!(
                        monkey_map.get(&1).unwrap().items,
                        vec![481, 245, 22, 26, 1092, 30]
                    );
                    assert_eq!(monkey_map.get(&2).unwrap().items, vec![]);
                    assert_eq!(monkey_map.get(&3).unwrap().items, vec![]);
                }
                _ => {}
            }
        }
    }
}
