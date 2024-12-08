use std::fs;
use std::sync::LazyLock;

use eyre::{OptionExt, Result};
use regex::Regex;

const DO: &str = "do()";

static MUL_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap());

fn sum_mul_insns(body: &str) -> i32 {
    MUL_RE
        .captures_iter(body)
        .map(|c| {
            c.iter().skip(1).map(|m| m.unwrap().as_str().parse::<i32>().unwrap()).product::<i32>()
        })
        .sum()
}

fn main() -> Result<()> {
    let mut args = std::env::args();
    let fname = args.nth(1).ok_or_eyre("filename was no provided")?;
    let body = fs::read_to_string(fname.as_str())?;
    let total = sum_mul_insns(body.as_str());
    println!("{total}");
    let mut cond_total = 0i32;
    let mut input = body.as_str();
    while !input.is_empty() {
        let stop = input.find("don't()").unwrap_or(input.len());
        cond_total += sum_mul_insns(&input[..stop]);
        input = &input[stop..];
        let start = input.find(DO).map(|i| i + DO.len()).unwrap_or(input.len());
        input = &input[start..];
    }
    println!("{cond_total}");
    Ok(())
}
