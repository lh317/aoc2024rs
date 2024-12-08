use std::fs;

use eyre::{Context, OptionExt, Result};

fn is_valid(report: &[i32]) -> bool {
    report
        .windows(2)
        .map(|w| w[0].checked_sub(w[1]).filter(|r| r.abs() >= 1 && r.abs() <= 3).map(i32::signum))
        .try_fold(0, |acc, r| r.filter(|s| acc == 0 || acc == *s))
        .is_some()
}

fn main() -> Result<()> {
    let mut args = std::env::args();
    let fname = args.nth(1).ok_or_eyre("filename was no provided")?;
    let body = fs::read_to_string(fname.as_str())?;
    let mut valid: i32 = 0;
    let mut dampened: i32 = 0;
    for (lineno, line) in body.lines().enumerate() {
        let lineno = lineno + 1;
        let report = line
            .split_whitespace()
            .map(|p| p.parse().wrap_err_with(|| format!("{fname}:{lineno}: error parsing number")))
            .collect::<Result<Vec<i32>>>()?;
        if is_valid(&report) {
            valid += 1;
        } else {
            for i in 0..report.len() {
                let removed = report
                    .iter()
                    .enumerate()
                    .filter_map(|(j, r)| {
                        if i != j {
                            Some(*r)
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<i32>>();
                if is_valid(&removed) {
                    dampened += 1;
                    break;
                }
            }
        }
    }
    println!("{valid}");
    println!("{}", valid + dampened);
    Ok(())
}
