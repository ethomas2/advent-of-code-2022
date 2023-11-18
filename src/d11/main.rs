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
    lhs: Either<Old, i32>,
    rhs: Either<Old, i32>,
    op: Op,
}

impl Operation {
    fn call(&self, old_item: i32) -> i32 {
        let [lhs, rhs]: [i32; 2] = [self.lhs, self.rhs].map(|x| match x {
            Either::Left(Old) => old_item,
            Either::Right(y) => y,
        });

        match self.op {
            Op::Add => lhs + rhs,
            Op::Sub => lhs - rhs,
            Op::Mul => lhs * rhs,
        }
    }
}

#[derive(Debug)]
struct Monkey {
    id: i32,
    items: Vec<i32>,
    operation: Operation,
    divisible: i32,
    if_true: i32,
    if_false: i32,
}

impl Monkey {
    fn test(&self, item: i32) -> i32 {
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
        cap.name("id").ok_or("bad id")?.as_str().parse::<i32>()?,
        cap.name("items").ok_or(":o")?.as_str(),
        cap.name("operation").ok_or(":o")?.as_str(),
        cap.name("divisible").ok_or(":o")?.as_str().parse::<i32>()?,
        cap.name("if_true").ok_or(":o")?.as_str().parse::<i32>()?,
        cap.name("if_false").ok_or(":o")?.as_str().parse::<i32>()?,
    );

    let items = items
        .split(' ')
        .map(|s| s.replace(',', "").parse::<i32>().unwrap()) // TODO: get rid of unwrap
        .collect::<Vec<i32>>();

    let isnum = |x: &str| -> bool { x.chars().all(|c| c.is_numeric()) };
    let operation: Operation = {
        let pieces = operation.split(" ").collect::<Vec<&str>>();
        let op = match pieces[1] {
            "+" => Op::Add,
            "-" => Op::Sub,
            "*" => Op::Mul,
            _ => panic!("unexpected piece {}", pieces[1]),
        };
        let [lhs, rhs]: [Either<Old, i32>; 2] = [pieces[0], pieces[2]].map(|x| match x {
            "old" => Either::Left(Old),
            _ if isnum(x) => Either::Right(x.parse::<i32>().expect(":o")),
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

type MonkeyMap = HashMap<i32, Monkey>;

fn take_turn<F>(map: &mut MonkeyMap, id: i32, mut closure: F)
where
    F: FnMut(&MonkeyMap),
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
        closure(map);
    }
}

fn take_round<F>(map: &mut MonkeyMap, mut closure: F)
where
    F: FnMut(&MonkeyMap),
{
    for id in 0..map.len() {
        take_turn(map, id as i32, &mut closure);
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string("src/d11/input")?;
    let mut monkey_map = parse(content.as_str())?;
    let mut num = 0;
    let mut closure = |mm: &MonkeyMap| {
        num += 1;
    };
    for _ in 0..20 {
        take_round(&mut monkey_map, &mut closure);
    }
    println!("{}", num);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn _get_data() -> MonkeyMap {
    let mut monkey_map: MonkeyMap = HashMap::new();

    // Monkey 0
    monkey_map.insert(0, Monkey {
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
    });

    // Monkey 1
    monkey_map.insert(1, Monkey {
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
    });

    // Monkey 2
    monkey_map.insert(2, Monkey {
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
    });

    // Monkey 3
    monkey_map.insert(3, Monkey {
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
    });
    return monkey_map;
    }


    fn test_1() {
        let mut mm: MonkeyMap = HashMap::new();
        Monkey { id: 0, items:
    }
}
