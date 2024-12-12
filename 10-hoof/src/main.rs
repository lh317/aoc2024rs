use std::collections::{HashSet, VecDeque};
use std::fs;

use eyre::{eyre, OptionExt, Result};
use ndarray::Array2;
use petgraph::algo::simple_paths::all_simple_paths;
use petgraph::graphmap::DiGraphMap;

fn in_bounds((y, x): (isize, isize), (rows, cols): (isize, isize)) -> bool {
    y >= 0 && y < rows && x >= 0 && x < cols
}

fn main() -> Result<()> {
    let mut args = std::env::args();
    let fname = args.nth(1).ok_or_eyre("filename was no provided")?;
    let body = fs::read_to_string(fname.as_str())?;
    let mut columns = 0isize;
    let mut rows = 0isize;
    let mut values = Vec::new();
    let mut zeros = Vec::new();
    for line in body.lines() {
        let lineno = rows + 1;
        for (col, c) in line.chars().enumerate() {
            let height =
                c.to_digit(10).ok_or_else(|| eyre!("{fname}:{lineno}: {c} is not a digit"))? as u8;
            values.push(height);
            if height == 0 {
                zeros.push((rows, col as isize))
            }
        }
        if rows == 0 {
            columns = line.len() as isize;
        } else if line.len() != columns as usize {
            return Err(eyre!("{fname}:{lineno}: line not of expected length {columns}"));
        }
        rows += 1;
    }
    if rows == 0 || columns == 0 {
        return Err(eyre!("{fname}: lines were empty"));
    }
    let map = Array2::from_shape_vec((rows as usize, columns as usize), values)?;
    let mut score = 0;
    let mut ratings = 0usize;
    for zero in zeros {
        let mut nines = HashSet::new();
        let mut graph = DiGraphMap::new();
        graph.add_node(zero);
        let mut queue = VecDeque::from([zero]);
        while let Some((y, x)) = queue.pop_front() {
            let height = map[(y as usize, x as usize)];
            if height == 9 {
                nines.insert((y, x));
            } else {
                let adjacent = [(y, x + 1), (y, x - 1), (y - 1, x), (y + 1, x)];
                for (y1, x1) in adjacent {
                    if in_bounds((y1, x1), (rows, columns))
                        && map[(y1 as usize, x1 as usize)] == height + 1
                    {
                        queue.push_back((y1, x1));
                        graph.add_node((y1, x1));
                        graph.add_edge((y, x), (y1, x1), 1);
                    }
                }
            }
        }
        score += nines.len();
        ratings += nines
            .into_iter()
            .flat_map(|nine| all_simple_paths::<Vec<_>, _>(&graph, zero, nine, 8, Some(8)))
            .count()
    }
    println!("{score}");
    println!("{ratings}");
    Ok(())
}
