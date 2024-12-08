use std::fs;

use eyre::{eyre, OptionExt, Result};

fn main() -> Result<()> {
    let mut args = std::env::args();
    let fname = args.nth(1).ok_or_eyre("filename was no provided")?;
    let body = fs::read_to_string(fname.as_str())?;
    let mut left = vec![];
    let mut right = vec![];
    for (lineno, line) in body.lines().enumerate() {
        let lineno = lineno + 1;
        for (i, part) in line.split_whitespace().enumerate() {
            match i {
                0 => left.push(part.parse::<i32>()?),
                1 => right.push(part.parse::<i32>()?),
                _ => Err(eyre!("{fname}:{lineno}: too many numbers"))?,
            };
        }
    }
    left.sort();
    right.sort();
    let sum: i32 = left.iter().zip(right.iter()).map(|(l, r)| (l - r).abs()).sum();
    println!("{}", sum);
    let similiarity: i32 = left.iter().try_fold(0, |acc, l| {
        i32::try_from(right.iter().filter(|r| l == *r).count()).map(|r| acc + l * r)
    })?;
    println!("{}", similiarity);
    Ok(())
}
