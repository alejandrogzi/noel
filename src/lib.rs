//! A small library for computing the number of base pairs in a gene
//!
//! to include noel as a library and use it within your project follow these steps:
//! 1. include `noel = 0.2.1` or `noel = "*"` under `[dependencies]` in your `Cargo.toml` file or just run `cargo add noel` from the command line
//! 2. the library name is `noel`, to use it just write:
//! ``` rust
//! use noel::{noel, noel_reader};
//! ```
//! or
//! ``` rust
//! use noel::*;
//! ```
//! 3. invoke
//! ``` rust
//! let exons = noel_reader(input: &PathBuf)
//! let lengths: HashMap<String, u32> = noel(exons: HashMap<String, Vec<(u32, u32)>>)
//! ```

use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read};
use std::path::PathBuf;

use rayon::prelude::*;

pub mod record;
pub use record::*;

pub fn get_lengths(exons: HashMap<String, Vec<(u32, u32)>>) -> Option<Vec<(String, u32)>> {
    let lengths = exons
        .into_par_iter()
        .map(|(gene, exons)| {
            let (min_start, max_end) = exons.iter().fold((u32::MAX, 0), |acc, &(start, end)| {
                (acc.0.min(start), acc.1.max(end))
            });

            let mut bp = vec![0; (max_end - min_start + 1) as usize];

            for &(start, end) in exons.iter() {
                for i in (start - min_start)..(end - min_start + 1) {
                    bp[i as usize] = 1;
                }
            }

            let total_bp: u32 = bp.iter().sum();
            (gene, total_bp)
        })
        .collect::<Vec<(String, u32)>>();

    Some(lengths)
}

pub fn noel_reader(input: &PathBuf) -> Option<HashMap<String, Vec<(u32, u32)>>> {
    let contents = reader(input).unwrap();
    let records = parallel_parse(&contents).unwrap();

    let exons: HashMap<String, Vec<(u32, u32)>> = records
        .into_par_iter()
        .filter(|line| line.feature() == "exon") // Filter out non-exons
        .fold(
            || HashMap::new(),
            |mut map, line| {
                map.entry(line.gene_id)
                    .or_insert_with(Vec::new)
                    .push((line.start, line.end));
                map
            },
        )
        .reduce(
            || HashMap::new(),
            |mut exons, thread_map| {
                for (key, value) in thread_map {
                    exons.entry(key).or_insert_with(Vec::new).extend(value);
                }
                exons
            },
        );

    Some(exons)
}

fn reader(file: &PathBuf) -> io::Result<String> {
    let mut file = File::open(file)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn parallel_parse<'a>(s: &'a str) -> Result<Vec<Record>, NoelError> {
    let records: Result<Vec<Record>, NoelError> = s
        .par_lines()
        .map(|line| match Record::new(line) {
            Ok(record) => Ok(record),
            Err(_) => Ok(Record {
                feat: "".to_string(),
                start: 0,
                end: 0,
                gene_id: "".to_string(),
            }),
        })
        .collect();

    return records;
}
