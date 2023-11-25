use std::error::Error;
use std::fs;

#[derive(Debug)]
enum Packet {
    List(Vec<Packet>),
    Number(i32),
}

impl PartialEq for Packet {
    fn eq(&self, other: &Packet) -> bool {
        self.partial_cmp(other).unwrap() == std::cmp::Ordering::Equal
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Packet) -> Option<std::cmp::Ordering> {
        use Packet::*;
        match (self, other) {
            (Number(x), Number(y)) => Some(x.cmp(y)),
            (List(l1), List(l2)) => {
                for (x, y) in l1.iter().zip(l2.iter()) {
                    match x.partial_cmp(y) {
                        Some(std::cmp::Ordering::Less) => return Some(std::cmp::Ordering::Less),
                        Some(std::cmp::Ordering::Greater) => {
                            return Some(std::cmp::Ordering::Greater)
                        }
                        _ => {}
                    }
                }
                if l1.len() != l2.len() {
                    return l1.len().partial_cmp(&l2.len());
                }
                return Some(std::cmp::Ordering::Equal);
            }
            (l1 @ List(_), Number(n)) => l1.partial_cmp(&List(vec![Number(*n)])),
            (Number(n), l2 @ List(_)) => List(vec![Number(*n)]).partial_cmp(l2),
        }
    }
}

mod parse {
    use super::Packet;

    type ParseError = String;

    fn find_close_brace(s: &[char]) -> Result<usize, ParseError> {
        if s[0] != '[' {
            return Err("expected opening brace".to_owned());
        }

        let mut n = 0;
        for i in 0..(s.len()) {
            match s[i] {
                '[' => n += 1,
                ']' => n -= 1,
                _ => (),
            }
            if n == 0 {
                return Ok(i);
            }
        }
        Err("oh no".to_owned())
    }

    pub fn parse_packet(s: &[char]) -> Result<(Packet, usize), ParseError> {
        if s[0].is_digit(10) {
            let mut i = 0;
            while s[i].is_digit(10) && i < s.len() {
                i += 1;
            }
            let n: i32 = s[0..i].iter().collect::<String>().parse().unwrap();
            return Ok((Packet::Number(n), i));
        } else if s[0] == '[' {
            let close_bracket_idx = find_close_brace(s)?;
            let mut idx = 1;

            let mut packets = Vec::new();
            while idx < close_bracket_idx {
                let (next_packet, offset) = parse_packet(&s[idx..])?;
                idx = idx + offset;
                packets.push(next_packet);
                if s[idx] == ',' {
                    idx += 1
                } else if idx == close_bracket_idx {
                    // do nothing
                } else {
                    return Err("yall done fucked up".to_owned());
                }
            }
            if idx != close_bracket_idx {
                return Err("yall done fucked up".to_owned());
            }
            idx += 1;
            Ok((Packet::List(packets), idx))
        } else {
            Err(format!("Unexpected character {}", s[0]))
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string("src/d13/input")?;
    let mut lines = content.lines();
    let mut i = 1;
    let mut sum = 0;
    loop {
        let (p1, _) = parse::parse_packet(&lines.next().unwrap().chars().collect::<Vec<char>>())?;
        let (p2, _) = parse::parse_packet(&lines.next().unwrap().chars().collect::<Vec<char>>())?;
        if p1 <= p2 {
            sum += i;
        }
        match lines.next() {
            Some(_) => {}
            None => break,
        }
        i += 1;
    }

    println!("Sum {}", sum);

    Ok(())
}
