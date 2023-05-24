use std::error::Error;
use std::fs;

fn from_snafu(snafu: &str) -> isize {
    let l = snafu.len();
    assert!(l > 0);
    let mut total: isize = 0;
    for i in 0..l {
        total *= 5;
        let c = snafu.chars().nth(i).unwrap();
        total += match c {
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                c.to_digit(10).unwrap() as isize
            }
            '-' => -1,
            '=' => -2,
            _ => panic!("Bad input"),
        };
    }
    return total;
}

fn to_snafu(n: isize) -> String {
    if n == 0 {
        return String::from("0");
    }
    let ndigits: u32 = {
        let mut ndigits = 1;
        let mut m = 1; // 2*m is the max number you can acheive with n digits
        loop {
            if 2 * m >= n.abs() {
                break;
            }
            ndigits += 1;
            m = 5 * m + 1;
        }
        ndigits
    };

    let (first_digit_as_char, first_digit_as_num) = {
        let m: isize = if ndigits >= 2 {
            2_isize * (0..=(ndigits - 2)).map(|ii| 5_isize.pow(ii)).sum::<isize>()
        } else {
            0
        };
        if n >= 2 * 5_isize.pow(ndigits - 1) - m {
            ('2', 2)
        } else if n >= 5_isize.pow(ndigits - 1) - m {
            ('1', 1)
        } else if n >= 0 - m {
            ('0', 0)
        } else if n >= -5_isize.pow(ndigits - 1) - m {
            ('-', -1)
        } else if n >= -2 * 5_isize.pow(ndigits - 1) - m {
            ('=', -2)
        } else {
            panic!(":o")
        }
    };

    if ndigits == 1 {
        first_digit_as_char.to_string()
    } else {
        let remain = to_snafu(n - (first_digit_as_num * 5_isize.pow(ndigits - 1)));
        let nzeroes = ndigits - (remain.len() as u32) - 1;
        let zeroes = '0'.to_string().repeat(nzeroes as usize);
        return first_digit_as_char.to_string() + &zeroes + &remain;
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string("src/d25/input")?;
    let total: isize = content.lines().map(from_snafu).sum();
    let snafu = to_snafu(total);
    println!("{}", snafu);
    Ok(())
}
