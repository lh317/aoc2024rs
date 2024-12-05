use std::cmp::{max, min};
use std::fs;

use eyre::{OptionExt, Result};
use ndarray::{s, Array, Array2};

const MAS: &str = "MAS";
const XMAS: &str = "XMAS";

fn main() -> Result<()> {
    let xmas = Array::from_iter(XMAS.chars());
    let samx = Array::from_iter(XMAS.chars().rev());
    let mas = Array::from_iter(MAS.chars());
    let sam = Array::from_iter(MAS.chars().rev());

    let mut args = std::env::args();
    let fname = args.nth(1).ok_or_eyre("filename was no provided")?;
    let body = fs::read_to_string(fname.as_str())?;
    let mut columns = 0isize;
    let mut rows = 0isize;
    let mut values = Vec::new();
    for line in body.lines() {
        rows += 1;
        for c in line.chars() {
            values.push(c)
        }
        if rows == 1 {
            columns = values.len() as isize;
        }
    }
    let puzzle = Array2::from_shape_vec((rows as usize, columns as usize), values)?;
    let mut count = 0;
    let mut xmas_count = 0;
    for ((row, col), c) in puzzle.indexed_iter() {
        let row = row as isize;
        let col = col as isize;
        if *c == 'X' {
            // Right
            if puzzle.slice(s![row, col..min(col + 4, columns)]) == xmas
            {
                count += 1;
            }
            // Left
            if puzzle.slice(s![row, max(0, col - 3)..col + 1]) == samx
            {
                count += 1;
            }
            // Down
            if puzzle.slice(s![row..min(row + 4, rows), col]) == xmas
            {
                count += 1;
            }
            // Up
            if puzzle.slice(s![max(0, row - 3)..row + 1, col]) == samx
            {
                count += 1;
            }
            // L-R-D
            // X
            //  M
            //   A
            //    S
            if puzzle.slice(s![row..min(row + 4, rows), col..min(col + 4, columns)]).diag() == xmas {
                count += 1;
            }
            // R-L-U
            // S
            //  A
            //   M
            //    X
            if puzzle.slice(s![max(0, row-3)..row+1, max(0, col-3)..col+1]).diag() == samx {
                count += 1;
            }
            // L-R-U
            //    S
            //   A
            //  M
            // X
            // Reverse row order to make L-R-D
            if puzzle.slice(s![max(0, row-3)..row+1;-1, col..min(col + 4, columns)]).diag() == xmas {
                count += 1;
            }
            // R-L-D
            //    X
            //   M
            //  A
            // S
            // Reverse column order to make L-R-D.
            if puzzle.slice(s![row..min(row + 4, rows), max(0, col-3)..col+1;-1]).diag() == xmas {
                count += 1;
            }
        }
        if *c == 'A' {
            // L-R-D
            // M
            //  A
            //   S
            let lrd = puzzle.slice(s![max(0, row-1)..min(row +2,rows), max(0, col-1)..min(col+2, columns)]).diag() == mas;
            // R - L -U
            // S
            //  A
            //   M
            let rlu = puzzle.slice(s![max(0, row-1)..min(row +2,rows), max(0, col-1)..min(col+2, columns)]).diag() == sam;
            // L - R -U
            //   S
            //  A
            // M
            let lru = puzzle.slice(s![max(0, row-1)..min(row +2,rows);-1, max(0, col-1)..min(col+2, columns)]).diag() == mas;
            // R-L-D
            //   M
            //  A
            // S
            let rld = puzzle.slice(s![max(0, row-1)..min(row +2,rows), max(0, col-1)..min(col+2, columns);-1]).diag() == mas;
            if (lrd || rlu) && (lru || rld)  {
                xmas_count += 1;
            }
        }
    }
    println!("{count}");
    println!("{xmas_count}");
    Ok(())
}
