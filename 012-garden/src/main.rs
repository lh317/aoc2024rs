use std::collections::HashSet;
use std::fs;

use eyre::{eyre, OptionExt, Result};
use ndarray::Array2;
use petgraph::graph::{UnGraph, NodeIndex};
use petgraph::algo::kosaraju_scc;

fn in_bounds(y: isize, x: isize, rows: isize, cols: isize) -> bool {
    y >= 0 && y < rows && x >= 0 && x < cols
}

fn to_offset(y: isize, x: isize, cols: isize) -> u32 {
    ((y * cols) + x) as u32
}

fn to_rc(offset: isize, cols: isize) -> (isize, isize) {
    (offset / cols, offset % cols)
}

fn main() -> Result<()> {
    let mut args = std::env::args();
    let fname = args.nth(1).ok_or_eyre("filename was no provided")?;
    let body = fs::read_to_string(fname.as_str())?;
    let mut columns = 0isize;
    let mut rows = 0isize;
    let mut values = Vec::new();
    for line in body.lines() {
        for c in line.chars() {
            values.push(c);
        }
        if rows == 0 {
            columns = line.len() as isize;
        } else if line.len() != columns as usize {
            return Err(eyre!("{fname}:{}: line not of expected length {columns}", rows + 1));
        }
        rows += 1;
    }
    let garden = Array2::from_shape_vec((rows as usize, columns as usize), values)?;
    let mut graph = UnGraph::<u32, ()>::with_capacity(garden.len(), garden.len()*3);
    for _ in 0..garden.len() {
        graph.add_node(0u32);
    }
    for (pos, c) in garden.indexed_iter() {
        let y = pos.0 as isize;
        let x = pos.1 as isize;
        let offset = to_offset(y, x, columns);
        let iter = [(y - 1, x), (y + 1, x), (y, x - 1), (y, x + 1)].into_iter().filter_map(|(y1, x1)| {
            if in_bounds(y1, x1, rows, columns) && garden[(y1 as usize, x1 as usize)] == *c {
                Some((offset, to_offset(y1, x1, columns)))
            } else {
                None
            }
        });
        let result = iter.clone();
        graph[NodeIndex::new(offset as usize)] = 4 - (iter.count() as u32);
        graph.extend_with_edges(result);
    }
    let plots = kosaraju_scc(&graph);
    let price: usize = plots.iter().map(|p| p.len() * p.iter().map(|n| graph[*n]).sum::<u32>() as usize).sum();
    println!("{price}");
    let mut bulk_price = 0;
    for plot in plots {
        let (py, px) = to_rc(plot.first().unwrap().index() as isize, columns);
        let plant = garden[(py as usize, px as usize)];
        //println!("{plant}::");
        let mut lines = HashSet::new();
        let mut cols = HashSet::new();
        for offset in plot.iter() {
            let (y, x) = to_rc(offset.index() as isize, columns);
            lines.insert(y);
            cols.insert(x);
        }
        let mut sides = 0;
        for y in lines {
            //println!("{y}: {sides}");
            let mut is_above = false;
            let mut is_below = false;
            for x in 0..columns {
                let p = garden[(y as usize, x as usize)];
                if p == plant && plot.contains(&NodeIndex::new(to_offset(y, x, columns) as usize)) {
                    if !is_above && (!in_bounds(y - 1, x, rows, columns) || garden[((y - 1) as usize, x as usize)] != plant) {
                        is_above = true;
                        sides += 1;
                    }
                    if is_above && (in_bounds(y -1, x, rows, columns) && garden[((y -1) as usize, x as usize)] == plant) {
                        is_above = false;
                    }
                    if !is_below && (!in_bounds(y + 1, x, rows, columns) || garden[((y + 1) as usize, x as usize)] != plant) {
                        is_below = true;
                        sides += 1;
                    }
                    if is_below && (in_bounds(y + 1, x, rows, columns) && garden[((y + 1) as usize, x as usize)] == plant) {
                        is_below = false;
                    }
                } else {
                    is_above = false;
                    is_below = false;
                }
            }
            println!("{y}: {sides}");
        }
        //println!("now going down");
        for x in cols {
            //println!("{x}: {sides}");
            let mut is_left = false;
            let mut is_right = false;
            for y in 0..rows {
                let p = garden[(y as usize, x as usize)];
                if p == plant && plot.contains(&NodeIndex::new(to_offset(y, x, columns) as usize)) {
                    if !is_left && (!in_bounds(y, x - 1, rows, columns) || garden[(y as usize, (x - 1) as usize)] != plant) {
                        //println!("match L");
                        is_left = true;
                        sides += 1;
                    }
                    if is_left && (in_bounds(y, x - 1, rows, columns) && garden[(y as usize, (x - 1) as usize)] == plant) {
                        is_left = false;
                    }
                    if !is_right && (!in_bounds(y, x + 1, rows, columns) || garden[(y as usize, (x + 1) as usize)] != plant) {
                        //println!("match R");
                        is_right = true;
                        sides += 1;
                    }
                    if is_right && (in_bounds(y, x + 1, rows, columns) && garden[(y as usize, (x + 1) as usize)] == plant) {
                        is_right = false;
                    }
                } else {
                    is_left = false;
                    is_right = false;
                }
            }
            println!("{x}: {sides}");
        }
            // let (above, below, _) = garden.row(index as usize).indexed_iter().fold((0, 0, None), |(above, below, lc), (x, c)| match (lc, c) {
            //     (None, c) if *c == plant => {
            //         let a = !in_bounds(index - 1, x as isize, rows, columns) || garden[((index - 1) as usize, x)] != plant;
            //         let b = !in_bounds(index + 1, x as isize, rows, columns) || garden[((index + 1 ) as usize, x)] != plant;
            //         (above + a as usize, below + b as usize, Some(*c))
            //     },
            //     (Some(lc), c) if lc != plant && *c == plant => {
            //         let a = !in_bounds(index - 1, x as isize, rows, columns) || garden[((index - 1) as usize, x)] != plant;
            //         let b = !in_bounds(index + 1, x as isize, rows, columns) || garden[((index + 1 ) as usize, x)] != plant;
            //         (above + a as usize, below + b as usize, Some(*c))
            //     },
            //     _ => (above, below, Some(*c)),
            // });
            //sides += above + below;
        // for index in cols {
        //     let (left, right, _) = garden.column(index as usize).indexed_iter().fold((0, 0, None), |(left, right, lc), (y, c)| match (lc, c) {
        //         (None, c) if *c == plant => {
        //             let l = !in_bounds(y as isize, index - 1, rows, columns) || garden[(y, (index - 1) as usize)] != plant;
        //             let r = !in_bounds(y as isize, index + 1, rows, columns) || garden[(y, (index + 1 ) as usize)] != plant;
        //             (left + l as usize, right + r as usize, Some(*c))
        //         },
        //         (Some(lc), c) if lc != plant && *c == plant => {
        //             let l = !in_bounds(y as isize, index - 1, rows, columns) || garden[(y, (index - 1) as usize)] != plant;
        //             let r = !in_bounds(y as isize, index + 1, rows, columns) || garden[(y, (index + 1 ) as usize)] != plant;
        //             (left + l as usize, right + r as usize, Some(*c))
        //         },
        //         _ => (left, right, Some(*c)),
        //     });
        //     sides += left +right;
        // }
        println!("{plant}: {sides}, {}", plot.len());
        bulk_price += sides * plot.len();
    }
    println!("{bulk_price}");
    Ok(())
}
