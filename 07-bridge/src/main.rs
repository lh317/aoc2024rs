use std::fs;
//use std::ops::{Add, Mul};
use std::str::FromStr;

use eyre::{eyre, Context, OptionExt, Report, Result};
use itertools::Itertools;

#[derive(Debug)]
enum Operator {
    Plus,
    Multiply,
    Concatenate
}

impl Operator {
    fn apply(&self, lhs: i64, rhs: i64) -> i64 {
        match self {
            Operator::Plus => lhs + rhs,
            Operator::Multiply => lhs * rhs,
            Operator::Concatenate => (lhs.to_string() + &rhs.to_string()).parse::<i64>().unwrap()
        }
    }
}


#[derive(Debug)]
struct Equation {
    result: i64,
    terms: Vec<i64>,
}

impl Equation {
    fn is_solvable(&self, operators: &[Operator]) -> bool {
        let num_operators = self.terms.len() - 1;
        for ops in (0..num_operators).map(|_| operators).multi_cartesian_product() {
            let mut ops_iter = ops.iter();
            let total = self.terms.iter().copied().reduce(|acc, rhs|{
                let op = ops_iter.next().unwrap();
                op.apply(acc, rhs)
            }).unwrap();
            if total == self.result {
                return true;
            }
        }
        false
    }
}
impl FromStr for Equation {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (result_str, rest) = s
            .split_once(':')
            .ok_or_else(|| eyre!("{s}: missing delimiter '|'"))?;
        let result = result_str.parse::<i64>()?;
        let terms = rest
            .split_whitespace()
            .map(|t| t.parse::<i64>())
            .collect::<Result<Vec<_>, _>>()?;
        if terms.len() < 2 {
            return Err(eyre!("need at least 2 terms, found: {}", terms.len()));
        }
        Ok(Equation { result, terms })
    }
}

fn main() -> Result<()> {
    let mut args = std::env::args();
    let fname = args.nth(1).ok_or_eyre("filename was no provided")?;
    let body = fs::read_to_string(fname.as_str())?;
    let equations = body
        .lines()
        .enumerate()
        .map(|(lineno, l)| {
            l.parse::<Equation>()
                .wrap_err_with(|| format!("{fname}:{}", lineno + 1))
        })
        .collect::<Result<Vec<_>, _>>()?;
    let first_cal: i64 = equations.iter().filter_map(|eq| if eq.is_solvable(&[Operator::Plus, Operator::Multiply]) { Some(eq.result)} else {None}).sum();
    println!("{first_cal}");
    let second_cal: i64 = equations.iter().filter_map(|eq| if eq.is_solvable(&[Operator::Plus, Operator::Multiply, Operator::Concatenate]) { Some(eq.result)} else {None}).sum();
    println!("{second_cal}");
    Ok(())
}
