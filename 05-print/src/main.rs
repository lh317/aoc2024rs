use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs;
use std::str::FromStr;

use eyre::{eyre, OptionExt, Report, Result};

#[derive(Debug, Clone, Copy)]
struct PageOrder {
    before: i32,
    page: i32
}

impl FromStr for PageOrder {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (before_str, page_str) = s.split_once('|').ok_or_else(|| eyre!("{s}: missing delimiter '|'"))?;
        let before = before_str.parse()?;
        let page = page_str.parse()?;
        Ok(PageOrder {before, page})
    }
}

fn main() -> Result<()> {
    let mut args = std::env::args();
    let fname = args.nth(1).ok_or_eyre("filename was no provided")?;
    let body = fs::read_to_string(fname.as_str())?;
    let mut orders = HashMap::new();
    let mut lines = body.lines();
    for line in lines.by_ref() {
        if line.trim().is_empty() {
            break;
        }
        let order = line.parse::<PageOrder>()?;
        orders.entry(order.page).or_insert_with(Vec::new).push(order.before);
    }
    let mut jobs = Vec::new();
    for line in lines {
        let mut job = Vec::new();
        for s in line.split(',') {
            job.push(s.parse::<i32>()?);
        }
        jobs.push(job);
    }
    let middle: i32 = jobs.iter().map(|j| {
        let ordered = j.iter().enumerate().all(|(i, p)| {
            if let Some(deps) = orders.get(p) {
                j[i+1..].iter().all(|np| !deps.contains(np))
            } else {
                true
            }
        });
        if ordered {
            j[j.len()/2]
        } else {
            0
        }
    }).sum();
    println!("{middle}");
    let (correct, incorrect) = jobs.iter_mut().fold((0i32, 0i32), |(corr, incorr), j| {
        let ordered = j.iter().enumerate().all(|(i, p)| {
            if let Some(deps) = orders.get(p) {
                j[i+1..].iter().all(|np| !deps.contains(np))
            } else {
                true
            }
        });
        if ordered {
            (corr + j[j.len()/2], incorr)
        } else {
            let mut full_deps: HashMap<i32, Vec<i32>> = HashMap::new();
            for p in j.iter() {
                if let Some(deps) = orders.get(p) {
                    let mut deps_for_p = Vec::from_iter(deps.iter().copied().filter(|p| j.contains(p)));
                    let mut i = 0;
                    while i < deps_for_p.len() {
                        if let Some(deps) = orders.get(&deps_for_p[i]) {
                            let mut to_append = Vec::from_iter(deps.iter().copied().filter(|p| j.contains(p) && !deps_for_p.contains(p)));
                            deps_for_p.append(&mut to_append);
                        }
                        i += 1;
                    }
                    full_deps.insert(*p, deps_for_p);
                }
            }
            j.sort_by(|lhs, rhs| {
                if full_deps.get(lhs).map_or(false, |d| d.contains(rhs)) {
                    Ordering::Greater
                } else if full_deps.get(rhs).map_or(false, |d| d.contains(lhs)) {
                    Ordering::Less
                } else {
                    lhs.cmp(rhs)
                }
            });
            (corr, incorr + j[j.len()/2])
        }
    });
    println!("{correct} {incorrect}");
    Ok(())
}
