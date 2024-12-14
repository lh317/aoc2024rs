use std::collections::HashMap;
use std::fs;

use eyre::{eyre, Context, OptionExt, Result};

fn main() -> Result<()> {
    let mut args = std::env::args();
    let fname = args.nth(1).ok_or_eyre("filename was no provided")?;
    let body = fs::read_to_string(fname.as_str())?;
    let mut pebbles = HashMap::new();
    for num_str in body.split_whitespace() {
        let p = num_str.parse::<u64>().wrap_err_with(|| format!("could not parse {num_str} as integer"))?;
        *pebbles.entry(p).or_insert(0u64) += 1;
    }
    for _ in 0..25 {
        let mut next = HashMap::with_capacity(pebbles.capacity());
        for (k, v) in pebbles.into_iter() {
            if k == 0 {
                *next.entry(1u64).or_insert(0u64) += v;
            } else {
                let base = k.ilog10();
                if base % 2 == 1 {
                    let divisor = 10u64.pow((base + 1) / 2);
                    let left = k / divisor;
                    let right = k % divisor;
                    *next.entry(left).or_insert(0u64) += v;
                    *next.entry(right).or_insert(0u64) += v;
                } else {
                    let new = k.checked_mul(2024).ok_or_else(|| eyre!("{k} * 2024 overflows"))?;
                    *next.entry(new).or_insert(0u64) += v;
                }
            }
        }
        pebbles = next;
    }
    println!("{}", pebbles.values().sum::<u64>());
    for _ in 25..75 {
        let mut next = HashMap::with_capacity(pebbles.capacity());
        for (k, v) in pebbles.into_iter() {
            if k == 0 {
                *next.entry(1u64).or_insert(0u64) += v;
            } else {
                let base = k.ilog10();
                if base % 2 == 1 {
                    let divisor = 10u64.pow((base + 1) / 2);
                    let left = k / divisor;
                    let right = k % divisor;
                    *next.entry(left).or_insert(0u64) += v;
                    *next.entry(right).or_insert(0u64) += v;
                } else {
                    let new = k.checked_mul(2024).ok_or_else(|| eyre!("{k} * 2024 overflows"))?;
                    *next.entry(new).or_insert(0u64) += v;
                }
            }
        }
        pebbles = next;
    }
    println!("{}", pebbles.values().sum::<u64>());
    Ok(())
}
