use std::cmp::{min, max};
use std::fs;

use enumset::{EnumSet, EnumSetType};
use eyre::{eyre, OptionExt, Result};
use ndarray::{s, Array, Array2};

#[derive(EnumSetType, Debug, PartialOrd, Ord, Hash)]
enum Direction {
    Up,
    Right,
    Down,
    Left
}

impl Direction {
    fn next(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up
        }
    }
}

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
    let mut start = (-1isize, -1isize);
    for (lineno, line) in body.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            match c {
                '^' => { start = (rows, col as isize); values.push(false)},
                '#' => values.push(true),
                _ => values.push(false),
            };
        }
        if rows == 0 {
            columns = values.len() as isize;
        } else if line.len() != columns as usize {
            return Err(eyre!("{fname}:{lineno}: line not of expected length {columns}"));
        }
        rows += 1;
    }
    if start == (-1, -1) {
        return Err(eyre!("{fname}: starting position '^' never found"));
    }

    let map = Array2::from_shape_vec((rows as usize, columns as usize), values)?;
    let mut visited = Array::<EnumSet<Direction>, _>::default(map.raw_dim());
    let mut guard_pos = start;
    let mut dir = Direction::Up;
    while in_bounds(guard_pos, (rows, columns)) {
        let (y, x) = guard_pos;
        let slice = match dir {
            Direction::Up => { s![0..y+1;-1, x]},
            Direction::Down => { s![y..rows, x]},
            Direction::Left => { s![y, 0..x+1;-1]},
            Direction::Right => {s![y, x..columns]},
        };
        let path = map.slice(slice);
        let obs = path.iter().position(|&p| p).unwrap_or(path.dim() + 1) as isize;
        let (new_y, new_x) = match dir {
            Direction::Up => (y - (obs - 1), x),
            Direction::Down => (y + (obs - 1), x),
            Direction::Left => (y, x - (obs - 1)),
            Direction::Right => (y, x + (obs - 1)),
        };
        let to_obs = s![0..min(obs as usize, path.dim())];
        visited.slice_mut(slice).slice_mut(to_obs).map_inplace(|es| {es.insert(dir);});
        guard_pos = (new_y, new_x);
        dir = dir.next();
    }
    let visited_count = visited.iter().filter(|&&p| !p.is_empty()).count();
    println!("{visited_count}");
    let block_count = visited.indexed_iter().filter(|(block_pos, es)| {
        if es.is_empty() {
            return false;
        }
        let block_y = block_pos.0 as isize;
        let block_x = block_pos.1 as isize;
        let mut map = map.clone();
        let mut visited = Array::<EnumSet<Direction>, _>::default(map.raw_dim());
        map[*block_pos] = true;
        guard_pos = start;
        dir = Direction::Up;
        let mut hit_obs = false;
        while in_bounds(guard_pos, (rows, columns)) {
            let (y, x) = guard_pos;
            let slice = match dir {
                Direction::Up => { s![0..y+1;-1, x]},
                Direction::Down => { s![y..rows, x]},
                Direction::Left => { s![y, 0..x+1;-1]},
                Direction::Right => {s![y, x..columns]},
            };
            let path = map.slice(slice);
            let obs = path.iter().position(|&p| p).unwrap_or(path.dim() + 1) as isize;
            let to_obs = s![0..min(obs as usize, path.dim())];
            let (new_y, new_x) = match dir {
                Direction::Up => (y - (obs - 1), x),
                Direction::Down => (y + (obs - 1), x),
                Direction::Left => (y, x - (obs - 1)),
                Direction::Right => (y, x + (obs - 1)),
            };
            if !hit_obs {
                let obs_pos = match dir {
                    Direction::Up => (y - obs, x),
                    Direction::Down => (y + obs, x),
                    Direction::Left => (y, x - obs),
                    Direction::Right => (y, x + obs),
                };
                if obs_pos == (block_y, block_x) {
                    hit_obs = true;
                }
            } else {
                let is_loop = visited.slice(slice).slice(to_obs).iter().any(|es| es.contains(dir));
                if is_loop {
                    println!("Found hit using {block_pos:?} going {dir:?} at {new_y},{new_x}");
                    return true;
                }
            }
            visited.slice_mut(slice).slice_mut(to_obs).map_inplace(|es| {es.insert(dir);});
            guard_pos = (new_y, new_x);
            dir = dir.next();
        }
        false
    }).count();
    println!("{block_count}");
    Ok(())
}
