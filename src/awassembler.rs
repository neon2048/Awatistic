use std::{io::BufRead, str::FromStr};

use crate::{
    awacpu::{awascii::ord, Awatism},
    errors::AwawaError,
};

pub fn awassemble<R: BufRead>(src: R, dst: &mut Vec<Awatism>) -> Result<(), AwawaError> {
    for line in src.lines() {
        if let Ok(line) = line {
            let mut res = handle_line(&line)?;
            dst.append(&mut res);
        }
    }

    return Ok(());
}

fn handle_line(line: &str) -> Result<Vec<Awatism>, AwawaError> {
    let mut in_q = false;
    let mut in_dq = false;

    let s: String = line
        .trim_start()
        .chars()
        .take_while(|x| {
            let x = *x;

            if x == '\'' && !in_dq {
                in_q = !in_q;
            } else if x == '"' && !in_q {
                in_dq = !in_dq;
            }

            if !in_q && !in_dq && x == '#' {
                return false;
            }

            return true;
        })
        .collect(); // Remove comments

    let (awatism_s, awatism_args) = match s.find(char::is_whitespace) {
        None => (s.as_str(), None),
        Some(idx) => (&s[..idx], Some(s[idx..].trim())),
    };

    if !awatism_s.is_empty() {
        let awatisms = string_to_awatism(awatism_s, awatism_args)?;
        return Ok(awatisms);
    }

    return Ok(vec![]);
}

pub fn print_awatisms<I>(awas: Vec<Awatism>, out: &mut I) -> std::fmt::Result
where
    I: std::fmt::Write,
{
    write!(out, "awa")?;
    for awa in awas {
        print_awatism(awa, out)?;
    }
    return Ok(());
}

fn print_awatism<I>(awa: Awatism, out: &mut I) -> std::fmt::Result
where
    I: std::fmt::Write,
{
    let n = awa.discriminant();

    print_awawa(n, 5, out)?;
    match awa {
        Awatism::Blo(val) => print_awawa(val as u8, 8, out),
        Awatism::Sbm(val) => print_awawa(val, 5, out),
        Awatism::Srn(val) => print_awawa(val, 5, out),
        Awatism::Lbl(val) => print_awawa(val, 5, out),
        Awatism::Jmp(val) => print_awawa(val, 5, out),
        _ => Ok(()),
    }
}

fn print_awawa<I>(awa: u8, n: u8, out: &mut I) -> std::fmt::Result
where
    I: std::fmt::Write,
{
    for i in 0..n {
        if awa & (1 << (n - i - 1)) != 0 {
            write!(out, "wa")?;
        } else {
            write!(out, " awa")?;
        }
    }
    return Ok(());
}

fn get<R>(args: Option<&str>) -> Result<R, AwawaError>
where
    R: FromStr,
{
    let n = match args {
        None => return Err(AwawaError::MissingArgumentError),
        Some(x) => x,
    };

    match n.parse::<R>() {
        Ok(i) => return Ok(i),
        Err(_) => return Err(AwawaError::InvalidArgumentError),
    }
}

fn string_to_awatism(s: &str, args: Option<&str>) -> Result<Vec<Awatism>, AwawaError> {
    let res = match s.to_lowercase().as_str() {
        "nop" => vec![Awatism::Nop],
        "prn" => vec![Awatism::Prn],
        "pr1" => vec![Awatism::Pr1],
        "red" => vec![Awatism::Red],
        "r3d" => vec![Awatism::R3d],
        "blo" => {
            let args = match args {
                None => return Err(AwawaError::MissingArgumentError),
                Some(x) => x.replace("\\n", "\n"),
            };

            if args.starts_with('\'') && args.ends_with('\'') && args.len() >= 3 {
                let c = args
                    .chars()
                    .nth(1)
                    .expect("bounds checked so element 1 should always exist");
                match ord(c) {
                    None => return Err(AwawaError::InvalidAwasciiCharError(c)),
                    Some(x) => vec![Awatism::Blo(x as i8)],
                };
            }

            if args.starts_with('\"') && args.ends_with('\"') {
                let awatisms: Result<Vec<Awatism>, AwawaError> = args[1..args.len() - 1]
                    .chars()
                    .rev()
                    .map(|el| match ord(el) {
                        None => Err(AwawaError::InvalidAwasciiCharError(el)),
                        Some(x) => Ok(Awatism::Blo(x as i8)),
                    })
                    .collect();
                return awatisms;
            }

            match args.parse::<i8>() {
                Err(_) => return Err(AwawaError::InvalidArgumentError),
                Ok(i) => vec![Awatism::Blo(i)],
            }
        }
        "sbm" => {
            let arg: u8 = get(args)?;
            vec![Awatism::Sbm(arg)]
        }
        "pop" => vec![Awatism::Pop],
        "dpl" => vec![Awatism::Dpl],
        "srn" => {
            let arg: u8 = get(args)?;
            vec![Awatism::Srn(arg)]
        }
        "mrg" => vec![Awatism::Mrg],
        "add" => vec![Awatism::Add],
        "sub" => vec![Awatism::Sub],
        "mul" => vec![Awatism::Mul],
        "div" => vec![Awatism::Div],
        "cnt" => vec![Awatism::Cnt],
        "lbl" => {
            let arg: u8 = get(args)?;
            vec![Awatism::Lbl(arg)]
        }
        "jmp" => {
            let arg: u8 = get(args)?;
            vec![Awatism::Jmp(arg)]
        }
        "eql" => vec![Awatism::Eql],
        "lss" => vec![Awatism::Lss],
        "gr8" => vec![Awatism::Gr8],
        "trm" => vec![Awatism::Trm],

        _ => return Err(AwawaError::UnknownAwatismError(String::from(s))),
    };
    return Ok(res);
}
