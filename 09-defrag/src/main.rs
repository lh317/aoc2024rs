use std::fs;
use std::cmp::Ordering;

use eyre::{eyre, OptionExt, Result};

#[derive(Debug, Clone, Copy)]
enum DiskExtent {
    File { id: u32, len: u8},
    Free {len: u8},
}

impl DiskExtent {

    fn len(&self) -> u8 {
        match self {
            DiskExtent::File{len, ..} => *len,
            DiskExtent::Free{len} => *len
        }
    }

    fn truncate(&mut self, rhs: u8) {
        *match self {
            DiskExtent::File{len, ..} => len,
            DiskExtent::Free{len} => len,
        } -= rhs;

    }
}

// fn is_fragmented(extents: &[DiskExtent]) -> bool {
//     extents.is_sorted_by(|lhs, rhs| !matches!((lhs, rhs), (DiskExtent::Free {..}, DiskExtent::File {..})))
// }

fn is_fragmented(extents: &[DiskExtent]) -> bool {
    extents.iter().skip_while(|ex| matches!(ex, DiskExtent::File{..})).any(|ex| matches!(ex, DiskExtent::File { ..}))
}
fn last_file_position(extents: &[DiskExtent]) -> Option<usize> {
    extents.iter().rev().position(|e| matches!(e, DiskExtent::File{..})).map(|i| extents.len() - 1 - i)
}

fn main() -> Result<()> {
    let mut args = std::env::args();
    let fname = args.nth(1).ok_or_eyre("filename was no provided")?;
    let body = fs::read_to_string(fname.as_str())?;
    let mut file = true;
    let mut disk = body.trim_end().chars().enumerate().map(|(i, c)| {
        let len = c.to_digit(10).unwrap() as u8;
        let result = if file {
            if len == 0 {
                return Err(eyre!("empty file"));
            }
            DiskExtent::File { id: u32::try_from(i/2)?, len }
        } else {
            DiskExtent::Free { len }
        };
        file = !file;
        Ok(result)
    }).filter(|ex| !matches!(ex, Ok(DiskExtent::Free{len}) if *len == 0)).collect::<Result<Vec<_>>>()?;
    if disk.len() < 2 {
        return Err(eyre!("{fname}: contained less than two entries"))
    }
    let mut second = disk.clone();
    let max_files = disk.iter().filter_map(|&ex| match ex {
        DiskExtent::File {id, ..} => Some(id),
        DiskExtent::Free { .. } => None,
    }).max().unwrap();
    let mut free_index = 1;
    while is_fragmented(&disk) {
        let mut to_fill = disk[free_index].len();
        while to_fill > 0 {
            let last_file_index = last_file_position(&disk).unwrap();
            let file_extent = disk[last_file_index];
            match file_extent {
                DiskExtent::File {id, len} => {
                    match to_fill.cmp(&len) {
                        Ordering::Less => {
                            disk[free_index] = DiskExtent::File {
                                id,
                                len: to_fill
                            };
                            disk[last_file_index].truncate(to_fill);
                            to_fill = 0;
                        },
                        Ordering::Equal => {
                            disk[free_index] = file_extent;
                            disk.remove(last_file_index);
                            to_fill = 0;
                        },
                        Ordering::Greater => {
                            disk[free_index] = file_extent;
                            disk[last_file_index] = DiskExtent::Free {
                                len
                            };
                            to_fill -= len;
                            free_index += 1;
                            disk.insert(free_index, DiskExtent::Free {len: to_fill});
                        },
                    };
                },
                _ => unreachable!(),
            };
        }
        free_index += disk[free_index..].iter().position(|e| matches!(e, DiskExtent::Free{..})).unwrap();
    }
    let checksum = disk.iter().fold((0u64, 0u64), |(block, checksum), &ex| match ex {
        DiskExtent::File {id, len} => {
            let end = block + (len as u64);
            (end, checksum + (block..end).map(|b| b * (id as u64)).sum::<u64>())
        },
        DiskExtent::Free{len} => (block + (len as u64), checksum),
    }).1;
    println!("{checksum}");
    for id in (0..max_files+1).rev() {
        let index = second.iter().position(|ex| matches!(ex, DiskExtent::File { id: fid, .. } if id == *fid)).unwrap();
        //println!("Defragmenting {id}: {index}");
        match second[index] {
            DiskExtent::File { id, len} => {
                let free_space = second[..index].iter().enumerate().find_map(|(i,ex)| match ex {
                    DiskExtent::Free { len: flen } if *flen >= len => Some((i, *flen)),
                    _ => None
                });
                if let Some((free_index, free_len)) = free_space {
                    //println!("Moving {id} to {free_index}");
                    second[free_index] = second[index];
                    second[index] = DiskExtent::Free { len };
                    if free_len > len {
                        second.insert(free_index + 1, DiskExtent::Free {len: free_len - len});
                    }
                }
            },
            DiskExtent::Free{..} => unreachable!()
        }
    }
    let checksum2 = second.iter().fold((0u64, 0u64), |(block, checksum), &ex| match ex {
        DiskExtent::File {id, len} => {
            let end = block + (len as u64);
            (end, checksum + (block..end).map(|b| b * (id as u64)).sum::<u64>())
        },
        DiskExtent::Free{len} => (block + (len as u64), checksum),
    }).1;
    println!("{checksum2}");
    Ok(())
}
