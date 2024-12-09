use std::collections::{HashMap, HashSet};
use std::cmp::max;
use std::fs;

use eyre::{eyre, OptionExt, Result};
use itertools::Itertools;

fn in_bounds((y, x): (isize, isize), (rows, cols): (isize, isize)) -> bool {
    y >= 0 && y < rows && x >= 0 && x < cols
}

fn main() -> Result<()> {
    let mut args = std::env::args();
    let fname = args.nth(1).ok_or_eyre("filename was no provided")?;
    let body = fs::read_to_string(fname.as_str())?;
    let mut columns = 0isize;
    let mut rows = 0isize;
    let mut values = HashMap::new();
    for (lineno, line) in body.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            if c != '.' {
                values.entry(c).or_insert_with(Vec::new).push((rows, col as isize));
            }
        }
        if rows == 0 {
            columns = line.len() as isize;
        } else if line.len() != columns as usize {
            return Err(eyre!("{fname}:{lineno}: line not of expected length {columns}"));
        }
        rows += 1;
    }
    let first = values
        .iter()
        .filter(|(_, ants)| ants.len() > 1)
        .flat_map(|(_, ants)| {
            ants.iter().combinations(2).flat_map(|points| {
                let rise = points[1].0 - points[0].0;
                let run = points[1].1 - points[0].1;
                let mut antinodes = Vec::new();
                let first = (points[0].0 - rise, points[0].1 - run);
                if in_bounds(first, (rows, columns)) {
                    antinodes.push(first);
                }
                let second = (points[0].0 + 2 * rise, points[0].1 + 2 * run);
                if in_bounds(second, (rows, columns)) {
                    antinodes.push(second);
                }
                antinodes
            })
        })
        .collect::<HashSet<_>>();
    println!("{}", first.len());
    let second = values
        .iter()
        .filter(|(_, ants)| ants.len() > 1)
        .flat_map(|(_, ants)| {
            ants.iter().combinations(2).flat_map(|points| {
                let rise = points[1].0 - points[0].0;
                let run = points[1].1 - points[0].1;
                let mut antinodes = Vec::new();
                let max_steps = max(rows, columns) + 1;
                for step in -max_steps..max_steps {
                    let p = (points[0].0 + step * rise, points[0].1 + step * run);
                    if in_bounds(p, (rows, columns)) {
                        antinodes.push(p);
                    }
                }
                antinodes
            })
        })
        .collect::<HashSet<_>>();
    println!("{}", second.len());
    Ok(())
}
